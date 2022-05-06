/// Represents the score multiplier region of a [Throw]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Multiplier {
    Single,
    Double,
    Triple,
}

impl Multiplier {
    /// Get the actual number to multiply the thrown number with
    pub fn factor(&self) -> u8 {
        match self {
            Multiplier::Single => 1,
            Multiplier::Double => 2,
            Multiplier::Triple => 3,
        }
    }
}

/// An error that might occur when using [Throw::new()]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InvalidThrowError {
    BullseyeTriple,
    InvalidNumber(u8),
}

impl std::fmt::Display for InvalidThrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidThrowError::BullseyeTriple => writeln!(f, "Bullseye cannot be a triple"),
            InvalidThrowError::InvalidNumber(val) => writeln!(f, "Throw has invalid value {val}"),
        }
    }
}

impl std::error::Error for InvalidThrowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Throw {
    Bullseye(Multiplier),
    Number(Multiplier, u8),
    Miss,
}

impl Throw {
    /// Create a new bullseye throw.
    ///
    /// # Returns
    ///
    /// - [Ok(Throw)] if Multiplier is [Multiplier::Single] [Multiplier::Double]
    /// - [Err(InvalidThrowError)] otherwise
    pub fn bullseye(multiplier: Multiplier) -> Result<Throw, InvalidThrowError> {
        match multiplier {
            Multiplier::Triple => Err(InvalidThrowError::BullseyeTriple),
            mult => Ok(Throw::Bullseye(mult)),
        }
    }

    /// Create a new number 1-20 throw.
    ///
    /// # Returns
    ///
    /// - [Ok(Throw)] if [number] is in range [1;20]
    /// - [Err(InvalidThrowError)] otherwise
    pub fn number(multiplier: Multiplier, number: u8) -> Result<Throw, InvalidThrowError> {
        match number {
            number if (1u8..21u8).contains(&number) => Ok(Throw::Number(multiplier, number)),
            number => Err(InvalidThrowError::InvalidNumber(number)),
        }
    }

    pub fn single(number: u8) -> Result<Throw, InvalidThrowError> {
        Self::number(Multiplier::Single, number)
    }

    pub fn double(number: u8) -> Result<Throw, InvalidThrowError> {
        Self::number(Multiplier::Double, number)
    }

    pub fn triple(number: u8) -> Result<Throw, InvalidThrowError> {
        Self::number(Multiplier::Triple, number)
    }

    /// Create a missed throw
    ///
    /// # Returns
    ///
    /// - [Ok(Throw::Miss)]
    pub fn miss() -> Result<Throw, InvalidThrowError> {
        Ok(Throw::Miss)
    }

    /// Calculate the score of the throw.
    pub fn points(&self) -> u8 {
        match self {
            Throw::Miss => 0,
            Throw::Bullseye(mult) => 25 * mult.factor(),
            Throw::Number(mult, number) => mult.factor() * number,
        }
    }

    pub fn multiplier(&self) -> Option<Multiplier> {
        match self {
            Throw::Bullseye(mult) => Some(*mult),
            Throw::Number(mult, _) => Some(*mult),
            Throw::Miss => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triple_20_is_valid() {
        let throw = Throw::number(Multiplier::Triple, 20);

        assert_eq!(throw, Ok(Throw::Number(Multiplier::Triple, 20)));
    }

    #[test]
    fn triple_bullseye_is_not_valid() {
        let throw = Throw::bullseye(Multiplier::Triple);
        assert_eq!(throw, Err(InvalidThrowError::BullseyeTriple));
    }

    #[test]
    fn invalid_numbers_lead_to_err() {
        for number in [0u8, 21, 26] {
            let throw = Throw::number(Multiplier::Single, number);

            assert_eq!(throw, Err(InvalidThrowError::InvalidNumber(number)));
        }
    }

    #[test]
    fn score_is_calculated_correctly() {
        let score = Throw::number(Multiplier::Triple, 20).unwrap().points();
        assert_eq!(score, 60);
    }

    #[test]
    fn miss_has_score_zero() {
        let score = Throw::miss().unwrap().points();
        assert_eq!(score, 0)
    }

    #[test]
    fn bullseye_scores_correct() {
        let score = Throw::bullseye(Multiplier::Single).unwrap().points();
        assert_eq!(score, 25);

        let score = Throw::bullseye(Multiplier::Double).unwrap().points();
        assert_eq!(score, 50);
    }
}
