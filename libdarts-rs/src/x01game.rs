use builder_pattern::Builder;

use crate::{player::Player, throw::Throw, turn::Turn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    player: Player,
    turns: Vec<Turn>,
}

impl Participant {
    pub fn new(player: &Player) -> Participant {
        Participant {
            player: player.clone(),
            turns: vec![],
        }
    }
}

fn is_valid_score(score: u32) -> Result<u32, ()> {
    if score > 1 && (score - 1) % 100 == 0 {
        Ok(score)
    } else {
        Err(())
    }
}

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq)]
pub struct X01Game {
    #[validator(is_valid_score)]
    pub score: u32,
    pub players: Vec<Participant>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct CurrentPlayer {
    index: usize,
    points: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AddThrowResult {
    Finished(Participant),
    Unfinished(X01GameTurn),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

    fn new(game: X01Game, next_player: usize) -> Option<Self> {
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

    fn next_turn(mut self) -> Self {
        let turn = std::mem::take(&mut self.turn);
        self.current_participant_mut().turns.push(turn);
        let next_player = (self.current.index + 1) % self.game.players.len();
        X01GameTurn::new(self.game, next_player).unwrap()
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

        let turn_points = self.turn.points();

        let new_points = turn_points + throw.points();

        self.turn.add_throw(throw).unwrap();

        match self.current.points.checked_sub(new_points.into()) {
            None => {
                self.turn.bust();
                AddThrowResult::Unfinished(self.next_turn())
            }
            Some(points) => {
                if points == 0 {
                    // WINNER
                    AddThrowResult::Finished(self.current_participant_mut().clone())
                } else {
                    AddThrowResult::Unfinished(if self.turn.num_throws() == 3 {
                        self.next_turn()
                    } else {
                        self
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::throw::Multiplier;

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
        let result = turn.add_throw(first_throw);

        if let AddThrowResult::Unfinished(turn) = result {
            let second_throw = Throw::number(Multiplier::Double, 20).unwrap();

            if let AddThrowResult::Unfinished(turn) = turn.add_throw(second_throw) {
                let last_throw = Throw::number(Multiplier::Single, 1).unwrap();

                if let AddThrowResult::Finished(part) = turn.add_throw(last_throw) {
                    assert_eq!(part.player, player);
                }
            }
        }

        panic!()
    }
}
