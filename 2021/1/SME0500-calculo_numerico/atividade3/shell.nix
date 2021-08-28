{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Texlive medium, com titlesec e xstring
    (texlive.combine {
      inherit (texlive) scheme-medium titlesec xstring minted fvextra pgfplots catchfile upquote framed;
    })
    # Octave
    (octave.withPackages (ps: with ps; [symbolic]))
    # Python e pygments
    (python39.withPackages (ps: with ps; [pygments]))
  ];
}
