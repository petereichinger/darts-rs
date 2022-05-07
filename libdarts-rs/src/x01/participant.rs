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
