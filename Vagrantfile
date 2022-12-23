# -*- mode: ruby -*-
# vi: set ft=ruby :

# Run cargo test on Linux

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/jammy64"
  config.vm.provision "shell", inline: <<~'SHELL'
    set -e
    if ! command -v cc >/dev/null ; then
      apt-get update
      apt-get install -y build-essential
    fi

    sudo -iu vagrant bash <<USER
      set -e
      if ! command -v rustup >/dev/null ; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
      fi
      source ~/.cargo/env
      rsync -a --exclude target /vagrant/ ~/git-status-vars/
      cd ~/git-status-vars/
      cargo test
    USER
  SHELL
end
