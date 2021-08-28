# Oblig2 de Web Semântica

Este programa irá ler um arquivo RDF (Em `.rdf`, `.ttl`, `.n3` ou `.nt`), deserializar, adicionar informações, adicionar informações da idade (`fam:Infant`, `fam:Minor` e `Fam:Old`), e depois serializar no seu arquivo de escolha (Também `.rdf`, `.ttl`, `.n3` ou `.nt`)

### Para compilar para production:
`cargo build --release`

### Como usar:
`target/release/oblig2 <arquivo_entrada> <arquivo_saída>`
