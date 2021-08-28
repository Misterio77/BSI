use aula03_ex8::SoccerTeam;

use rand::seq::SliceRandom;

fn main() {
    // Criar vetor de times de futebol
    let mut times = Vec::new();

    // Vou usar alguns nomes de times reais
    // Mas com dados meio inventados (pra forçar quase-empates)

    let sao_paulo = SoccerTeam {
        team_name: "São Paulo".into(),
        victories: 9,
        draws: 8,
        defeats: 2,
        goals_scored: 28,
        goals_conceded: 15,
        yellow_cards: 2,
        red_cards: 1,
    };
    times.push(sao_paulo);

    let santos = SoccerTeam {
        team_name: "Santos".into(),
        victories: 10,
        draws: 5,
        defeats: 1,
        goals_scored: 39,
        goals_conceded: 16,
        yellow_cards: 3,
        red_cards: 2,
    };
    times.push(santos);

    let palmeiras = SoccerTeam {
        team_name: "Palmeiras".into(),
        victories: 10,
        draws: 5,
        defeats: 2,
        goals_scored: 27,
        goals_conceded: 13,
        yellow_cards: 2,
        red_cards: 3,
    };
    times.push(palmeiras);

    let pte_preta = SoccerTeam {
        team_name: "Ponte Preta".into(),
        victories: 10,
        draws: 5,
        defeats: 2,
        goals_scored: 27,
        goals_conceded: 13,
        yellow_cards: 3,
        red_cards: 2,
    };
    times.push(pte_preta);

    let bragantino = SoccerTeam {
        team_name: "Bragantino".into(),
        victories: 10,
        draws: 5,
        defeats: 2,
        goals_scored: 27,
        goals_conceded: 13,
        yellow_cards: 3,
        red_cards: 2,
    };
    times.push(bragantino);

    let corinthians = SoccerTeam {
        team_name: "Corinthians".into(),
        victories: 7,
        draws: 3,
        defeats: 5,
        goals_scored: 24,
        goals_conceded: 19,
        yellow_cards: 1,
        red_cards: 1,
    };
    times.push(corinthians);

    // Eu optei por implementar o traço de ordenação sem aleatoriedade, para preservar
    // transitividade e assimetria. Para criar o desempate aleatório, vou primeiro misturar o vetor
    // de forma aleatória, e depois ordenar com algoritmo estável.
    //
    // Você pode verificar essa aleatoriedade rodando o programa e vendo a posição da ponte preta e
    // do bragantino, que (por terem números iguais) terão o aleatório como critério de desempate.
    times.shuffle(&mut rand::thread_rng());

    // Ordenar de maneira estável (vai ir em ordem crescente de pontos, e manterá aleatoriedade
    // como desempate final)
    times.sort();
    // Inverter ordenação (para poder exibir como ranking decrescente)
    times.reverse();

    println!("Times em ordem de colocação:");
    println!();
    for time in times {
        println!("{}", time);
        println!();
    }
}
