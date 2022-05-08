use builder_pattern::Builder;
use getset::Getters;

use crate::throw::{Multiplier, Throw};

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

#[derive(Builder, Debug, Clone, PartialEq, Eq, Getters)]
#[get = "pub"]
pub struct Ruleset {
    #[validator(is_valid_score)]
    pub(super) score: u32,
    #[default(InRule::Any)]
    pub(super) in_rule: InRule,
    #[default(OutRule::Any)]
    pub(super) out_rule: OutRule,
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn setting_x01_score_works() {
        for score in [101, 301, 501, 701, 1001] {
            assert!(Ruleset::new().score(score).is_ok());
        }
    }
    #[test]
    fn setting_score_works_correctly() {
        let ruleset = Ruleset::new().score(101).unwrap().build();

        assert_eq!(*ruleset.score(), 101u32);
    }

    #[test]
    fn game_with_invalid_score_fails() {
        let game = Ruleset::new().score(100);
        assert!(game.is_err());
    }

    #[test]
    fn game_with_score_1_fails() {
        let game = Ruleset::new().score(1);
        assert!(game.is_err());
    }
}
