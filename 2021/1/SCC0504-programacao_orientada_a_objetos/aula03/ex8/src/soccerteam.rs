use std::cmp::Ordering;
use std::fmt;

#[derive(Eq, PartialEq)]
pub struct SoccerTeam {
    pub team_name: String,
    pub victories: u16,
    pub defeats: u16,
    pub draws: u16,
    pub goals_scored: u16,
    pub goals_conceded: u16,
    pub yellow_cards: u16,
    pub red_cards: u16,
}

impl SoccerTeam {
    /// Obter placar (3*vitorias + 1*empates)
    pub fn get_score(&self) -> u16 {
        (self.victories * 3) + self.draws
    }
}

/// Implementação de ordenação (equivalente ao Comparable do Java)
impl Ord for SoccerTeam {
    fn cmp(&self, other: &Self) -> Ordering {
        // Como a comparação deve ser transitiva e assimetrica, eu não incluí a lógica de
        // aleatoriedade aqui. Ao invés disso, optei por sortear os times (lá na main) e fazer uma
        // ordenação estável. Obtendo assim o msm resultado e mantendo eficiencia da comparação.

        // Tentar com placar
        let score_cmp = self.get_score().cmp(&other.get_score());
        if score_cmp != Ordering::Equal {
            return score_cmp;
        }

        // Tentar com vitórias
        let victories_cmp = self.victories.cmp(&other.victories);
        if victories_cmp != Ordering::Equal {
            return victories_cmp;
        }

        // Tentar com número de gols
        let goals_cmp = self.goals_scored.cmp(&other.goals_scored);
        if goals_cmp != Ordering::Equal {
            return goals_cmp;
        }

        // Tentar com número de gols levados
        let goals_cmp = other.goals_conceded.cmp(&self.goals_conceded);
        if goals_cmp != Ordering::Equal {
            return goals_cmp;
        }

        // Tentar com número de cartões vermelhos
        let red_cards_cmp = other.red_cards.cmp(&self.red_cards);
        if red_cards_cmp != Ordering::Equal {
            return red_cards_cmp;
        }

        // Por fim, tentar com número de cartões amarelos
        other.yellow_cards.cmp(&self.yellow_cards)
    }
}

impl PartialOrd for SoccerTeam {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Implementação de display (print)
impl fmt::Display for SoccerTeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}pts\nVitórias/Empates/Derrotas: {}/{}/{}\nGols marcados/recebidos: {}/{}\nCartões vermelhos/amarelos: {}/{}", self.team_name, self.get_score(), self.victories, self.draws, self.defeats, self.goals_scored, self.goals_conceded, self.red_cards, self.yellow_cards)
    }
}
