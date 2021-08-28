{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    maven
    adoptopenjdk-hotspot-bin-11
    zip
    unzip
  ];
}
