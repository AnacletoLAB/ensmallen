# Nix shell for building the python wheels ready for publication
{ pkgs ? import <nixpkgs> {} }:

let
  rustupToolchain = "nightly-2024-06-13";
in
pkgs.mkShell {
  buildInputs = with pkgs; [ 
    python311

    rustup
    maturin

    stdenv
    pkg-config 
    openssl
    which
    gcc
    binutils

    # All the C libraries that a manylinux_1 wheel might depend on:
    ncurses
    xorg.libX11
    xorg.libXext
    xorg.libXrender
    xorg.libICE
    xorg.libSM
    glib
  ];

  RUST_BACKTRACE = 1;
  # use nightly bcz all the features we need are in nightly
  RUSTUP_TOOLCHAIN = rustupToolchain;
  # make everything self-contained to this folder
  CARGO_HOME = toString ./.cargo_home;
  RUSTUP_HOME = toString ./.rustup;

  CXX = "zig c++ -target x86_64-linux-gnu.2.16";
  CC  = "zig cc -target x86_64-linux-gnu.2.16";
}
