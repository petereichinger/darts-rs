use std::{error::Error, fmt::Display};

use super::throw::*;

#[derive(Debug, PartialEq, Eq)]
pub enum AddThrowError {
    MaximumThrowsReached,
}

impl Error for AddThrowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl Display for AddThrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddThrowError::MaximumThrowsReached => writeln!(f, "Maximum throws reached!"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Turn {
    throws: Vec<Throw>,
    maximum_throws: u8,
}

impl Default for Turn {
    fn default() -> Self {
        Self::new()
    }
}

impl Turn {
    pub fn new() -> Turn {
        Turn {
            throws: vec![],
            maximum_throws: 3,
        }
    }

    pub fn add_throw(&mut self, throw: Throw) -> Result<(), AddThrowError> {
        if self.throws.len() >= self.maximum_throws as usize {
            Err(AddThrowError::MaximumThrowsReached)
        } else {
            self.throws.push(throw);
            Ok(())
        }
    }

    pub fn score(&self) -> u8 {
        self.throws.iter().map(|t| t.score()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_hundred_eiiiighty() {
        let mut round = Turn::new();
        let triple_20 = Throw::number(Multiplier::Triple, 20).unwrap();

        (0..3).for_each(|_| {
            round.add_throw(triple_20.clone()).unwrap();
        });

        assert_eq!(round.score(), 180);
    }

    #[test]
    fn four_throws_are_invalid() {
        let mut round = Turn::new();

        let triple_20 = Throw::number(Multiplier::Triple, 20).unwrap();

        (0..3).for_each(|_| {
            round.add_throw(triple_20.clone()).unwrap();
        });

        assert_eq!(
            round.add_throw(triple_20),
            Err(AddThrowError::MaximumThrowsReached)
        )
    }
}
