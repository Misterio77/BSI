# Exercício 4 - Quests de RPG em Profundidade

## Sumário de mudanças
No Vertex, adicionei campos para X e Y, método de distância euclidiana, e método de igualidade,

No FloydWarshallTraversal, adicionei os métodos mostCentral, mostPeripheric, e farthestFrom. Esses contém toda a lógica para os outputs pedidos.

Assim como no exercício anterior, não inclui classes que não utilizei, e tirei funcionalidades do Graphviz para facilitar.

## Namespace e organização
Tentei utilizar o estilo de organização de namespace que eu costumo ver em projetos reais, com domínios invertidos.

Como eu tenho o domínio [misterio.me](https://misterio.me), usei o namespace `me.misterio.alg2brabalog`. Daí os arquivos de grafos ficam no namespace `me.misterio.alg2brabalog.graph`.

## Makefile e Maven
Requer java 11+ (não me preocupei em compatibilidade com 8, já que o run.codes não funciona)

Você pode compilar usando o Maven:
`maven package`

Ou o make:
`make build`

Em ambos os casos, a jar estará em `target`

## Nix (opcional)
Aqui tem um `nix.shell` que eu uso pra organizar meu ambiente de desenvolvimento.

Ele especifica quais pacotes meu gerenciador de pacotes ([Nix, do NixOS](https://nixos.org/)) usa nesse ambiente.

Imagino que não tem muita gente que use Nix, mas deixei aqui pq vaiii que. Só usar `nix-shell`.
