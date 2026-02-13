{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    gcc
    pkg-config # Essential for crates to find system libs
  ];
  buildInputs = with pkgs; [
    ncurses    # Fixes the "ncurses.h" error
    alsa-lib   # Needed for rodio/audio
    udev       # Often needed for input/gamepads
  ];
}
