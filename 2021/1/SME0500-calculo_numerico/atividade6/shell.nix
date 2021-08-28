{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Texlive medium, com titlesec e xstring
    (texlive.combine {
      inherit (texlive) scheme-medium titlesec xstring pgfplots;
    })
    # Octave
    (octave.withPackages (ps: with ps; [symbolic]))
  ];
}
