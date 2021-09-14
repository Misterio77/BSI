
{
  description = "Exercício SO";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
      name = "exercicio";
    in rec {

      packages.${name} = pkgs.stdenv.mkDerivation {
        pname = name;
        version = "1.0";
        src = ./.;
      };
      defaultPackage = packages.${name};

      apps.${name} = {
        type = "app";
        program = "${packages.${name}}/bin/${name}";
      };
      defaultApp = apps.${name};

      devShell =
        pkgs.mkShell { buildInputs = with pkgs; [ autoconf automake gnumake gcc valgrind gdb ]; };
    });
}
