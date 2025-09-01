install:
  cargo build --release
  sudo install -Dm755 target/release/elysium /usr/bin
  sudo install -Dm644 LICENSE /usr/share/licenses/elysium/LICENSE

uninstall:
  sudo rm /usr/bin/elysium
  sudo rm -r /usr/share/licenses/elysium
