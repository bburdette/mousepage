{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build`
          pname = "mousepage";
          packages.${pname} = naersk-lib.buildPackage {
            pname = pname;
            root = ./.;
            buildInputs = with pkgs; [
              cargo
              xorg.libXtst 
              libinput 
              xorg.libX11
              openssl.dev 
              pkgconfig
              nix
              ];
            # OPENSSL_LIB_DIR=openssl.dev;
          };
          defaultPackage = packages.${pname};

          # `nix run`
          apps.hello-world = flake-utils.lib.mkApp {
            drv = packages.hello-world;
          };
          defaultApp = apps.hello-world;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              rustc
              xorg.libXtst
              libinput 
              xorg.libX11
              openssl.dev 
              pkgconfig
              nix
              ];
            # OPENSSL_LIB_DIR=openssl.dev;
          };
        }
    );
}
