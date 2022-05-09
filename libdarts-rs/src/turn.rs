use super::throw::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Turn {
    throws: Vec<Throw>,
    bust: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ThrowError {
    Bust,
}

pub type ThrowResult = Result<(), ThrowError>;

impl Turn {
    pub fn new() -> Self {
        Turn {
            throws: vec![],
            bust: false,
        }
    }

    pub fn add_throw(&mut self, throw: Throw) -> ThrowResult {
        if self.bust {
            Err(ThrowError::Bust)
        } else {
            self.throws.push(throw);
            Ok(())
        }
    }

    pub fn num_throws(&self) -> usize {
        self.throws.len()
    }

    pub fn points(&self) -> u8 {
        if self.bust {
            0
        } else {
            self.throws.iter().map(|t| t.points()).sum()
        }
    }

    pub fn bust(&mut self) {
        self.bust = true;
    }

    pub fn is_bust(&self) -> bool {
        self.bust
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
            assert_eq!(Ok(()), round.add_throw(triple_20.clone()));
        });

        assert_eq!(round.points(), 180);
    }

    #[test]
    fn bust_is_set_correctly() {
        let mut turn = Turn::new();

        turn.bust();

        assert_eq!(turn.bust, true);
    }

    #[test]
    fn cant_add_throw_to_busted_turn() {
        let mut turn = Turn::new();

        turn.bust();

        assert_eq!(
            turn.add_throw(Throw::number(Multiplier::Triple, 20).unwrap()),
            Err(ThrowError::Bust)
        );
    }
}
