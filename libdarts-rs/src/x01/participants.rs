use std::ops::{Index, IndexMut};

use crate::player::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    pub player: Player,
}

impl Participant {
    pub fn new(player: &Player) -> Participant {
        Participant {
            player: player.clone(),
        }
    }
}

pub struct ParticipantsBuilder {
    participants: Vec<Participant>,
}

impl ParticipantsBuilder {
    fn new() -> Self {
        ParticipantsBuilder {
            participants: vec![],
        }
    }

    pub fn add(mut self, player: &Player) -> Self {
        self.participants.push(Participant {
            player: player.clone(),
        });

        Self {
            participants: self.participants,
        }
    }

    pub fn build(self) -> Option<Participants> {
        if self.participants.is_empty() {
            None
        } else {
            Some(Participants {
                participants: self.participants,
            })
        }
    }
}

#[cfg(test)]
pub fn test_participants(n: u8) -> Participants {
    let mut participants = Participants::new();

    if n > 0 {
        participants = participants.add(&Player::new("Anna").unwrap());
    }

    if n > 1 {
        participants = participants.add(&Player::new("Pete").unwrap());
    }

    participants.build().unwrap()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Participants {
    pub participants: Vec<Participant>,
}

impl Index<usize> for Participants {
    type Output = Participant;

    fn index(&self, index: usize) -> &Self::Output {
        &self.participants[index]
    }
}

impl IndexMut<usize> for Participants {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.participants[index]
    }
}

impl Participants {
    pub fn new() -> ParticipantsBuilder {
        ParticipantsBuilder::new()
    }

    pub fn count(&self) -> usize {
        self.participants.len()
    }
}
