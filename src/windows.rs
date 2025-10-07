//! Windows specific functionality.
//!
//! This is part of the executable, not the library; `unsafe` is allowed.

use super::params::Params;

/// Stub type to make clippy complain less.
pub struct Nothing;

/// Hook to process `Params`.
pub const fn os_params_hook(_params: &Params) -> Nothing {
    Nothing
}

/// Hook at normal process end.
pub const fn os_exit_hook(_: Nothing) {}
