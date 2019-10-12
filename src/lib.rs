pub enum MatchWinner {
    PlayerB = 0,
    PlayerA = 1,
}

pub struct EloRank {
    pub k: i32,
}

impl EloRank {
    fn calculate_expected(&self, score_a: f64, score_b: f64) -> f64 {
        1.0 / (1.0 + (10f64.powf((score_b - score_a) / 400.0)))
    }

    fn winner_to_score(&self, winner: MatchWinner) -> (f64, f64) {
        match winner {
            MatchWinner::PlayerA => (1.0, 0.0),
            MatchWinner::PlayerB => (0.0, 1.0),
        }
    }

    pub fn calculate(&self, player_a: f64, player_b: f64, winner: MatchWinner) -> (f64, f64) {
        let k = self.k as f64;

        let expected_a = self.calculate_expected(player_a, player_b);
        let expected_b = self.calculate_expected(player_b, player_a);

        println!("{} {}", player_a, expected_a);
        println!("{} {}", player_b, expected_b);

        let (score_a, score_b) = self.winner_to_score(winner);
        let new1 = player_a + k * (score_a - expected_a);
        let new2 = player_b + k * (score_b - expected_b);

        (new1.round(), new2.round())
    }
}

#[cfg(test)]
mod tests {
    use crate::{EloRank, MatchWinner};

    #[test]
    fn calculates_correct_ratings() {
        let elo = EloRank { k: 32 };
        let (player_a_new, player_b_new) = elo.calculate(1200.0, 1400.0, MatchWinner::PlayerA);
        assert_eq!(player_a_new, 1224.0);
        assert_eq!(player_b_new, 1376.0);

        let (player_a_new, player_b_new) = elo.calculate(1200.0, 1400.0, MatchWinner::PlayerB);
        assert_eq!(player_a_new, 1192.0);
        assert_eq!(player_b_new, 1408.0);

        let (player_a_new, player_b_new) = elo.calculate(1400.0, 1200.0, MatchWinner::PlayerA);
        assert_eq!(player_a_new, 1408.0);
        assert_eq!(player_b_new, 1192.0);

        let (player_a_new, player_b_new) = elo.calculate(1400.0, 1200.0, MatchWinner::PlayerB);
        assert_eq!(player_a_new, 1376.0);
        assert_eq!(player_b_new, 1224.0);
    }

    #[test]
    fn rounds_ratings_properly() {
        let elo = EloRank { k: 32 };
        let (player_a_new, player_b_new) = elo.calculate(1802.0, 1186.0, MatchWinner::PlayerA);
        assert_eq!(player_a_new, 1803.0);
        assert_eq!(player_b_new, 1185.0);
    }
}
