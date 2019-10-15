let
  # moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> {}; 
  # { overlays = [ moz_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "mousepage";
    buildInputs = [
      cargo
      xorg.libXtst 
      libinput 
      xorg.libX11
      openssl.dev 
      pkgconfig
      nix
      ];
    OPENSSL_LIB_DIR=openssl.dev;
  }
