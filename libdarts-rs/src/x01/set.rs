use std::{error::Error, fmt::Display};

use crate::throw::Throw;

use super::{
    leg::{self, Leg, ThrowResult},
    participants::Participants,
    ruleset::Ruleset,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Set<'a> {
    ruleset: &'a Ruleset,
    participants: &'a Participants,
    legs: Vec<Leg<'a>>,
    current_leg: Leg<'a>,
    first_player: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CreateSetError {
    InvalidFirstPlayer(usize),
}

impl Error for CreateSetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for CreateSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateSetError::InvalidFirstPlayer(n) => {
                writeln!(f, "First player '{}' is invalid.", n)
            }
        }
    }
}

impl<'a> Set<'a> {
    pub fn new(
        ruleset: &'a Ruleset,
        participants: &'a Participants,
        first_player: usize,
    ) -> Result<Self, CreateSetError> {
        if first_player >= participants.count() {
            Err(CreateSetError::InvalidFirstPlayer(first_player))
        } else {
            Ok(Self {
                ruleset,
                participants,
                legs: vec![],
                current_leg: Leg::new(ruleset, participants, first_player),
                first_player,
            })
        }
    }

    pub fn current_leg_number(&self) -> usize {
        self.legs.len() + 1
    }

    pub fn add_throw(mut self, throw: Throw) -> Self {
        let ThrowResult { state, leg } = self.current_leg.add_throw(throw);

        self.current_leg = match state {
            leg::State::Finished => {
                // TODO: Check if set is finished!
                self.first_player = (self.first_player + 1) % self.participants.count();
                self.legs.push(leg);
                Leg::new(self.ruleset, self.participants, self.first_player)
            }
            leg::State::Unfinished => leg,
        };

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::x01::{participants::test_participants, ruleset::Ruleset};

    use super::*;

    #[test]
    fn creating_set_with_invalid_first_participant_results_in_error() {
        let participants = test_participants(1);
        let ruleset = Ruleset::new().score(101).unwrap().build();

        let set = Set::new(&ruleset, &participants, 2);

        assert_eq!(set, Err(CreateSetError::InvalidFirstPlayer(2)));
    }
}
