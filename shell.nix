# shell.nix for Rust
{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "Rust";
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    pkg-config
  ];

  RUST_BACKTRACE = 1;
}
