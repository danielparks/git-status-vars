//! UNIX specific (or at least non-Windows) functionality.
//!
//! This is part of the executable, not the library; `unsafe` is allowed.

use nix::sys::signal::{
    kill, sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal,
};
use nix::unistd::{fork, getpid, write, ForkResult, Pid};
use std::thread;
use std::time::Duration;

use super::params::Params;

/// Hook to process `Params` — install the timeout.
pub fn os_params_hook(params: &Params) -> Option<Pid> {
    // Kludge. Clap doesn’t let a value parser return `Option<...>`:
    // https://github.com/clap-rs/clap/discussions/5320
    (params.timeout != Duration::ZERO).then(|| install_timeout(params.timeout))
}

/// Hook at normal process end — clean up the timeout child process.
pub fn os_exit_hook(pid: Option<Pid>) {
    if let Some(pid) = pid {
        tracing::debug!("Killing child {pid} used for timeout");
        // Ignore errors; the child will die on its own eventually.
        let _ = kill(pid, Signal::SIGTERM);
    }
}

/// Set up the timeout.
///
/// This uses [`alarm()`] and a signal handler to kill this process if it takes
/// too long (more than `timeout` seconds).
///
/// [`alarm()`]: https://man7.org/linux/man-pages/man2/alarm.2.html
fn install_timeout(timeout: Duration) -> Pid {
    let alarm_action = SigAction::new(
        SigHandler::Handler(sigalrm_handler),
        SaFlags::empty(),
        SigSet::empty(),
    );

    // Safety: see `sigaction` documentation:
    //
    //   1. The signal handler only uses async-signal-safe functions.
    //   2. We ignore the old signal handler.
    unsafe {
        let _ = sigaction(Signal::SIGALRM, &alarm_action);
    }

    let parent = getpid();

    // Safety: this is not a multi-threaded application, which is the only issue
    // mentioned in the `nix::unistd::fork()` docs.
    let child = match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => child,
        Ok(ForkResult::Child) => {
            // Not multithreaded, so non-async-safe syscalls are fine to use.
            thread::sleep(timeout);

            // Ignore errors. Most likely the parent process already exited.
            let _ = kill(parent, Signal::SIGALRM);

            // Don't run atexit handlers since this is the child process.
            // Safety: libc has no docs. Not multithreaded, so no issues there.
            unsafe {
                libc::_exit(0);
            }
        }
        Err(error) => panic!("Could not fork child for timeout: {error}"),
    };
    tracing::debug!("Forked child {child} to send SIGALRM after {timeout:?}");
    child
}

/// Signal handler for SIGALRM (for timeout).
///
/// This only calls [async-signal-safe] functions.
///
/// [async-signal-safe]: https://man7.org/linux/man-pages/man7/signal-safety.7.html
extern "C" fn sigalrm_handler(_: nix::libc::c_int) {
    // Start with a newline in case we were in the middle of printing a line.
    let _ = write(
        std::io::stdout(),
        b"\nrepo_state=Error\nrepo_error='Timed out'\n",
    );

    // Safety: some things might not be cleaned up. However:
    //
    //   1. We shouldn’t be making changes to the git repo.
    //   2. Even if we were making changes, git is resilient to being killed in
    //      the middle of an operation.
    unsafe {
        libc::_exit(2);
    }
}
