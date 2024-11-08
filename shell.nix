let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell {
  name = "allomancy";
  buildInputs = [
    rustToolchain
  ] ++ (with pkgs; [
    pkg-config
    probe-rs
    elf2uf2-rs
  ]);
  RUST_BACKTRACE = 1;
}