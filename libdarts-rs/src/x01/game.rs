use crate::{player::Player, throw::Throw, turn::Turn};

use super::{
    participant::{Participant, Participants},
    ruleset::Ruleset,
};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct CurrentPlayer {
    index: usize,
    points: u32,
    turn: Turn,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum State {
    Finished,
    Unfinished,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThrowResult {
    pub state: State,
    pub game: Game,
}

impl ThrowResult {
    fn unfinished(game: Game) -> ThrowResult {
        ThrowResult {
            state: State::Unfinished,
            game,
        }
    }
    fn finished(game: Game) -> ThrowResult {
        ThrowResult {
            state: State::Finished,
            game,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Game {
    ruleset: Ruleset,
    participants: Participants,
    current: CurrentPlayer,
}

impl Game {
    fn calculate_score(participant: &Participant, start_score: u32) -> Option<u32> {
        let sum = participant
            .turns
            .iter()
            .filter_map(|turn| {
                if turn.is_bust() {
                    None
                } else {
                    Some(turn.points() as u32)
                }
            })
            .sum();

        start_score.checked_sub(sum)
    }

    pub fn new(ruleset: Ruleset, participants: Participants) -> Self {
        Game {
            ruleset,
            participants,
            current: Default::default(),
        }
        .begin_turn(0)
    }

    fn begin_turn(self, next_player: usize) -> Self {
        let participant = &self.participants.participants[next_player];
        let points = Game::calculate_score(participant, *self.ruleset.score());

        if let Some(points) = points {
            Game {
                current: CurrentPlayer {
                    index: next_player,
                    points,
                    turn: Turn::new(),
                },
                ..self
            }
        } else {
            panic!("Invalid state reached")
        }
    }

    fn bust_turn(mut self) -> ThrowResult {
        self.current.turn.bust();
        self.next_turn()
    }

    fn next_turn(mut self) -> ThrowResult {
        let turn = std::mem::take(&mut self.current.turn);
        self.current_participant_mut().turns.push(turn);
        let next_player = (self.current.index + 1) % self.participants.participants.len();
        ThrowResult::unfinished(Game::begin_turn(self, next_player))
    }

    pub fn current_player(&self) -> &Player {
        &self.participants.participants[self.current.index].player
    }

    pub fn current_points(&self) -> u32 {
        self.current
            .points
            .checked_sub(self.current.turn.points().into())
            .unwrap()
    }

    fn current_participant_mut(&mut self) -> &mut Participant {
        &mut self.participants.participants[self.current.index]
    }

    pub fn add_throw(mut self, throw: Throw) -> ThrowResult {
        // Check if current throw results in new turn, win, continue turn, bust of turn

        let first_throw =
            self.current_participant_mut().turns.is_empty() && self.current.turn.num_throws() == 0;
        self.current.turn.add_throw(throw.clone()).unwrap();

        if first_throw && !self.ruleset.in_rule().valid_throw(&throw) {
            return self.bust_turn();
        }

        let turn_points = self.current.turn.points();

        match self.current.points.checked_sub(turn_points.into()) {
            None => self.bust_turn(), // Player has thrown more points than remain
            Some(points) => {
                if points == 0 {
                    if self.ruleset.out_rule().valid_finisher(&throw) {
                        ThrowResult::finished(self)
                    } else {
                        self.bust_turn()
                    }
                } else {
                    if self.ruleset.out_rule().valid_remaining_points(points) {
                        if self.current.turn.num_throws() == 3 {
                            self.next_turn()
                        } else {
                            ThrowResult::unfinished(self)
                        }
                    } else {
                        self.bust_turn()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::x01::game::State;
    use crate::x01::participant::Participants;
    use crate::x01::{game::ThrowResult, ruleset::Ruleset};
    use crate::{player::Player, throw::Throw};

    use super::Game;

    fn test_participants(n: u8) -> Participants {
        let mut participants = Participants::new();

        if n > 0 {
            participants = participants.add(&Player::new("Anna").unwrap());
        }

        if n > 1 {
            participants = participants.add(&Player::new("Pete").unwrap());
        }

        participants.build()
    }

    #[test]
    fn simple_game() {
        let participants = test_participants(1);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let game = Game::new(ruleset, participants);

        let first_throw = Throw::triple(20).unwrap();
        let second_throw = Throw::double(20).unwrap();
        let third_throw = Throw::single(1).unwrap();

        let ThrowResult { state, game } = game.add_throw(first_throw);

        assert_eq!(state, State::Unfinished);
        assert_eq!(game.current_points(), 41);

        let ThrowResult { state, game } = game.add_throw(second_throw);

        assert_eq!(state, State::Unfinished);
        assert_eq!(game.current_points(), 1);

        let ThrowResult { state, game } = game.add_throw(third_throw);

        assert_eq!(state, State::Finished);
        assert_eq!(game.current_points(), 0);
    }

    #[test]
    fn switching_players_works() {
        let participants = test_participants(2);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let mut game = Game::new(ruleset, participants.clone());

        let miss = Throw::miss().unwrap();

        assert_eq!(
            game.current_player().name(),
            participants.participants[0].player.name()
        );

        for _ in 0..3 {
            let ThrowResult {
                state: _,
                game: new_turn,
            } = game.add_throw(miss.clone());

            game = new_turn;
        }
        assert_eq!(
            game.current_player().name(),
            participants.participants[1].player.name()
        );
    }

    #[test]
    fn score_calculated_correctly() {
        let participants = test_participants(1);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let game = Game::new(ruleset, participants.clone());

        let miss = Throw::miss().unwrap();
        let d20 = Throw::double(20).unwrap();

        let ThrowResult { state: _, game } = game.add_throw(d20.clone());

        assert_eq!(game.current_points(), 61);

        let ThrowResult { state: _, game } = game.add_throw(miss.clone());
        let ThrowResult { state: _, game } = game.add_throw(miss.clone());

        assert_eq!(game.current_points(), 61);
    }

    #[test]
    fn score_is_calculated_correctyl_again_when_first_players_turn_again() {
        let participants = test_participants(2);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let game = Game::new(ruleset, participants.clone());

        let miss = Throw::miss().unwrap();
        let d20 = Throw::double(20).unwrap();

        let ThrowResult { state: _, game } = game.add_throw(d20.clone());

        assert_eq!(game.current_points(), 61);

        let ThrowResult { state: _, game } = game.add_throw(miss.clone());
        let ThrowResult { state: _, game } = game.add_throw(miss.clone());

        assert_eq!(game.current_points(), 101);

        let ThrowResult { state: _, game } = game.add_throw(miss.clone());
        let ThrowResult { state: _, game } = game.add_throw(miss.clone());
        let ThrowResult { state: _, game } = game.add_throw(miss.clone());

        assert_eq!(game.current_points(), 61);
    }

    #[test]
    fn next_player_after_bust() {
        let participants = test_participants(2);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let game = Game::new(ruleset, participants.clone());

        let t20 = Throw::triple(20).unwrap();

        let ThrowResult { state: _, game } = game.add_throw(t20.clone());
        let ThrowResult { state: _, game } = game.add_throw(t20.clone());

        assert_eq!(
            game.current_player().name(),
            participants.participants[1].player.name()
        );
    }

    #[test]
    fn bust_turn_is_added_corretly_to_participant() {
        let participants = test_participants(1);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let game = Game::new(ruleset, participants.clone());

        let t20 = Throw::triple(20).unwrap();

        let ThrowResult { state: _, game } = game.add_throw(t20.clone());
        let ThrowResult { state: _, game } = game.add_throw(t20.clone());

        assert_eq!(game.participants.participants[0].turns.len(), 1);
        assert_eq!(game.participants.participants[0].turns[0].is_bust(), true);
    }

    #[test]
    fn score_is_calculated_correctly_in_busted_turn() {
        let participants = test_participants(1);

        let ruleset = Ruleset::new().score(101).unwrap().build();

        let game = Game::new(ruleset, participants.clone());

        let t20 = Throw::triple(20).unwrap();

        let ThrowResult { state: _, game } = game.add_throw(t20.clone());
        let ThrowResult { state: _, game } = game.add_throw(t20.clone());

        assert_eq!(game.current_points(), 101);
    }
}
