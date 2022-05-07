use builder_pattern::Builder;

use crate::{
    player::Player,
    throw::{Multiplier, Throw},
};

use super::{participant::Participant, turn::X01GameTurn};

#[allow(dead_code)]
fn is_valid_score(score: u32) -> Result<u32, ()> {
    if score > 1 && (score - 1) % 100 == 0 {
        Ok(score)
    } else {
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InRule {
    Any,
    Double,
    Triple,
}

impl InRule {
    pub fn valid_throw(&self, throw: &Throw) -> bool {
        match self {
            InRule::Any => true,
            InRule::Double => throw.multiplier() == Some(Multiplier::Double),
            InRule::Triple => throw.multiplier() == Some(Multiplier::Triple),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutRule {
    Any,
    Double,
    Triple,
}

impl OutRule {
    pub fn valid_finisher(&self, throw: &Throw) -> bool {
        match self {
            OutRule::Any => true,
            OutRule::Double => throw.multiplier() == Some(Multiplier::Double),
            OutRule::Triple => throw.multiplier() == Some(Multiplier::Triple),
        }
    }

    pub fn valid_remaining_points(&self, remaining_points: u32) -> bool {
        match self {
            OutRule::Any => remaining_points >= 1,
            OutRule::Double => remaining_points >= 2,
            OutRule::Triple => remaining_points >= 3,
        }
    }
}

#[derive(Builder, Debug, Clone, PartialEq, Eq)]
pub struct X01Game {
    #[validator(is_valid_score)]
    pub score: u32,
    pub players: Vec<Participant>,
    #[default(InRule::Any)]
    pub in_rule: InRule,
    #[default(OutRule::Any)]
    pub out_rule: OutRule,
}

impl X01Game {
    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn players_iter(&self) -> impl Iterator<Item = &Player> {
        self.players.iter().map(|part| &part.player)
    }

    pub fn begin(self) -> X01GameTurn {
        X01GameTurn::new(self, 0).unwrap()
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_game_construction() {
        let game = X01Game::new()
            .score(101)
            .unwrap()
            .players(vec![
                Participant::new(&Player::new("Anna").unwrap()),
                Participant::new(&Player::new("Pete").unwrap()),
            ])
            .build();

        let mut players_iter = game.players_iter();
        let (p1, p2) = (players_iter.next().unwrap(), players_iter.next().unwrap());

        assert_eq!(p1, &Player::new("Anna").unwrap());
        assert_eq!(p2, &Player::new("Pete").unwrap());
    }

    #[test]
    fn game_with_invalid_score_fails() {
        let game = X01Game::new().score(100);
        assert!(game.is_err());
    }

    #[test]
    fn game_with_score_1_fails() {
        let game = X01Game::new().score(1);
        assert!(game.is_err());
    }
}
