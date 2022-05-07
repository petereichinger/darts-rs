use crate::{player::Player, throw::Throw, turn::Turn};

use super::{game::X01Game, participant::Participant};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CurrentPlayer {
    index: usize,
    points: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddThrowResult {
    Finished(X01Game),
    Unfinished(X01GameTurn),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct X01GameTurn {
    game: X01Game,
    current: CurrentPlayer,
    turn: Turn,
}

impl X01GameTurn {
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

    pub(super) fn new(game: X01Game, next_player: usize) -> Option<Self> {
        let participant = &game.players[next_player];
        let points = X01GameTurn::calculate_score(participant, game.score);

        match points {
            None => None,
            Some(points) => Some(X01GameTurn {
                game,
                current: CurrentPlayer {
                    index: next_player,
                    points,
                },
                turn: Turn::new(),
            }),
        }
    }

    fn bust_turn(mut self) -> AddThrowResult {
        self.turn.bust();
        self.next_turn()
    }

    fn next_turn(mut self) -> AddThrowResult {
        let turn = std::mem::take(&mut self.turn);
        self.current_participant_mut().turns.push(turn);
        let next_player = (self.current.index + 1) % self.game.players.len();
        AddThrowResult::Unfinished(X01GameTurn::new(self.game, next_player).unwrap())
    }

    pub fn current_player(&self) -> &Player {
        &self.game.players[self.current.index].player
    }

    pub fn current_points(&self) -> u32 {
        self.current
            .points
            .checked_sub(self.turn.points().into())
            .unwrap()
    }

    fn current_participant_mut(&mut self) -> &mut Participant {
        &mut self.game.players[self.current.index]
    }

    pub fn add_throw(mut self, throw: Throw) -> AddThrowResult {
        // Check if current throw results in new turn, win, continue turn, bust of turn

        let first_throw =
            self.current_participant_mut().turns.is_empty() && self.turn.num_throws() == 0;
        self.turn.add_throw(throw.clone()).unwrap();

        if first_throw && !self.game.in_rule.valid_throw(&throw) {
            return self.bust_turn();
        }

        let turn_points = self.turn.points();

        match self.current.points.checked_sub(turn_points.into()) {
            None => self.bust_turn(), // Player has thrown more points than remain
            Some(points) => {
                if points == 0 {
                    if self.game.out_rule.valid_finisher(&throw) {
                        AddThrowResult::Finished(self.game)
                    } else {
                        self.bust_turn()
                    }
                } else {
                    if self.game.out_rule.valid_remaining_points(points) {
                        if self.turn.num_throws() == 3 {
                            self.next_turn()
                        } else {
                            AddThrowResult::Unfinished(self)
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
    use crate::throw::Multiplier;

    use crate::x01::{game::X01Game, participant::Participant, turn::AddThrowResult};
    use crate::{player::Player, throw::Throw};

    #[test]
    fn simple_game() {
        let player = Player::new("Anna").unwrap();
        let participant = Participant::new(&player);

        let game = X01Game::new()
            .score(101)
            .unwrap()
            .players(vec![participant])
            .build();

        let turn = game.begin();

        let first_throw = Throw::number(Multiplier::Triple, 20).unwrap();

        if let AddThrowResult::Unfinished(turn) = turn.add_throw(first_throw) {
            assert_eq!(turn.current_points(), 41);

            let second_throw = Throw::number(Multiplier::Double, 20).unwrap();

            if let AddThrowResult::Unfinished(turn) = turn.add_throw(second_throw) {
                assert_eq!(turn.current_points(), 1);

                let last_throw = Throw::number(Multiplier::Single, 1).unwrap();

                if let AddThrowResult::Finished(_) = turn.add_throw(last_throw) {
                    return;
                }
            }
        }

        panic!()
    }
}
