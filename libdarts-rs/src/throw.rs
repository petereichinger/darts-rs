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

/// An error that might occur when using any of the methods to creat a throw
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

/// Typedef for the return value of the various creation methods of throws
pub type ThrowResult = Result<Throw, InvalidThrowError>;

/// Represents a single throw on the dart board
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Throw {
    /// The inner two rings of the dartboard, Multiplier indicates inner or outer bullseye
    Bullseye(Multiplier),
    /// One of the twenty numbers with multiplier
    Number(Multiplier, u8),
    /// Didn't hit any scoring region or the board at all
    Miss,
}

impl Throw {
    /// Create a new bullseye throw.
    ///
    /// # Returns
    ///
    /// - [Ok(Throw)] if Multiplier is [Multiplier::Single] [Multiplier::Double]
    /// - [Err(InvalidThrowError)] otherwise
    pub fn bullseye(multiplier: Multiplier) -> ThrowResult {
        match multiplier {
            Multiplier::Triple => Err(InvalidThrowError::BullseyeTriple),
            mult => Ok(Throw::Bullseye(mult)),
        }
    }

    /// Create a new number 1-20 throw.
    ///
    /// # Returns
    ///
    /// - [`Result::Ok([Throw])`] if [Throw::number] is in range \[1;20\]
    /// - [`Err(InvalidThrowError)`] otherwise
    pub fn number(multiplier: Multiplier, number: u8) -> ThrowResult {
        match number {
            number if (1u8..21u8).contains(&number) => Ok(Throw::Number(multiplier, number)),
            number => Err(InvalidThrowError::InvalidNumber(number)),
        }
    }

    /// Create a new single hit of a number
    ///
    /// Calls [Throw::number(Multiplier::Single, number)]
    pub fn single(number: u8) -> ThrowResult {
        Self::number(Multiplier::Single, number)
    }

    /// Create a new double hit of a number
    ///
    /// Calls [Throw::number(Multiplier::Double, number)]
    pub fn double(number: u8) -> ThrowResult {
        Self::number(Multiplier::Double, number)
    }

    /// Create a new triple hit of a number
    ///
    /// Calls [Throw::number(Multiplier::Triple, number)]
    pub fn triple(number: u8) -> ThrowResult {
        Self::number(Multiplier::Triple, number)
    }

    /// Create a missed throw
    ///
    /// # Returns
    ///
    /// - [Ok(Throw::Miss)]
    pub fn miss() -> ThrowResult {
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

    /// Get the multiplier if there is one
    ///
    /// # Returns
    ///
    /// - [OK(Multiplier)] if the throw is a number or bullseye throw
    /// - None in case of a miss
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
