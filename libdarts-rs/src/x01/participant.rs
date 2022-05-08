use crate::{player::Player, turn::Turn};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    pub player: Player,
    pub turns: Vec<Turn>,
}

impl Participant {
    pub fn new(player: &Player) -> Participant {
        Participant {
            player: player.clone(),
            turns: vec![],
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
            turns: vec![],
        });

        Self {
            participants: self.participants,
        }
    }

    pub fn build(self) -> Participants {
        Participants {
            participants: self.participants,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Participants {
    pub participants: Vec<Participant>,
}

impl Participants {
    pub fn new() -> ParticipantsBuilder {
        ParticipantsBuilder::new()
    }
}
