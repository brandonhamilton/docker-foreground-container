{ pkgs, ... }:

{
  enterShell = ''
    rustc --version
    echo "Profile binary path: $DEVENV_PROFILE/bin"
    echo "Rust source path: $RUST_SRC_PATH"
  '';

  languages.rust.enable = true;
}
