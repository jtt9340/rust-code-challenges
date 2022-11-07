let nixpkgs = import ./moz_overlay.nix;
in with nixpkgs;
stdenv.mkDerivation {
  name = "Rust Stable";

  buildInputs = [
    # Just selecting "rust" from latest.rustChannels.stable installs
    # the whole kit and caboodle: rustc, cargo, rustfmt, rust-gdb, etc...
    latest.rustChannels.stable.rust
    # GNU debugger
    gdb
  ];
}
