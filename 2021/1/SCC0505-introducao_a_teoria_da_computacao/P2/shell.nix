{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    # Texlive medium, com titlesec e xstring
    (texlive.combine {
      #inherit (texlive) scheme-medium titlesec xstring minted fvextra pgfplots catchfile upquote framed;
      inherit (texlive) scheme-medium xstring titlesec;
    })
  ];
}
