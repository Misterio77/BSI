# Exercício 4 - Quests de RPG em Profundidade

## Sumário de mudanças
Essas são as mudanças que o meu código tem para o [upstream de vocês](https://github.com/LeonardoTPereira/Graphs):
- Criei a classe `DepthFirstTraversal`, que herda `TravelStrategyInterface`.
- Na `DigraphList`, a ordem das arestas era a ordem de inserção, gerando divergências dos casos testes. Para manter o polimorfismo, sem coisa específicas desse digrafo na travessia, no método `DigraphList.getFirstConnectedVertex`, eu uso um `.sort` (com uma class `EdgeComparator` que eu criei, que compara arestas de acordo com o ID do vértice destino), para garantir que as arestas estão ordenadas.
- Adicionei campos ao `Vertex`, mudei seu `toString`, e adicinei um `getId` para acessar seu ID
- Adicionei quebras de linha no `printPath`, para bater com os casos teste.
- O código aqui não inclui as variantes (por exemplo, de grafo) que não utilizei.
- Dei uma podada em algumas features que serviam pra desenhar os gráficos, logs, e outras coisas que são maneiras mas não precisava aqui, pois fiquei um pouquinho confuso de trabalhar ao redor delas.

## Namespace e organização
Tentei utilizar o estilo de organização de namespace que eu costumo ver em projetos reais, com domínios invertidos.

Como eu tenho o domínio [misterio.me](https://misterio.me), usei o namespace `me.misterio.alg2quests`. Daí os arquivos de grafos ficam no namespace `me.misterio.alg2quests.graph`.

## Makefile e Maven
Você pode compilar usando o Maven:
`maven package`

Ou o make:
`make build`

Em ambos os casos, a jar estará em `target`

## Nix (opcional)
Aqui tem um `nix.shell` que eu uso pra organizar meu ambiente de desenvolvimento.

Ele especifica quais pacotes meu gerenciador de pacotes ([Nix, do NixOS](https://nixos.org/)) usa nesse ambiente.

Imagino que não tem muita gente que use Nix, mas deixei aqui pq vaiii que. Só usar `nix-shell`.
