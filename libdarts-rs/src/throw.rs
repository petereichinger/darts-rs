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
    InvalidValue(u8),
}

impl std::fmt::Display for InvalidThrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidThrowError::BullseyeTriple => writeln!(f, "Bullseye cannot be a triple"),
            InvalidThrowError::InvalidValue(val) => writeln!(f, "Throw has invalid value {val}"),
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
pub struct Throw {
    multiplier: Multiplier,
    number: u8,
}

impl Throw {
    fn valid_number(number: u8) -> bool {
        (1..=20).contains(&number) || number == 25
    }

    /// Construct a new Throw
    ///
    /// # Parameters
    ///
    /// - multiplier : Enum denoting the multipier region that was hit
    /// - number : Value that was hit
    ///
    /// # Returns
    ///
    /// A [Result] containing a successfully created [Throw] or an [InvalidThrowError] enum specifying what was wrong
    pub fn new(multiplier: Multiplier, number: u8) -> Result<Throw, InvalidThrowError> {
        match multiplier {
            Multiplier::Triple if number == 25 => Err(InvalidThrowError::BullseyeTriple),
            _ if !Throw::valid_number(number) => Err(InvalidThrowError::InvalidValue(number)),
            multiplier => Ok(Throw { multiplier, number }),
        }
    }

    pub fn score(&self) -> u8 {
        self.multiplier.factor() * self.number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triple_20_is_valid() {
        let throw = Throw::new(Multiplier::Triple, 20);

        assert_eq!(
            throw,
            Ok(Throw {
                multiplier: Multiplier::Triple,
                number: 20
            })
        );
    }

    #[test]
    fn triple_bullseye_is_not_valid() {
        let throw = Throw::new(Multiplier::Triple, 25);
        assert_eq!(throw, Err(InvalidThrowError::BullseyeTriple));
    }

    #[test]
    fn invalid_numbers_lead_to_err() {
        for number in [0u8, 21, 26] {
            let throw = Throw::new(Multiplier::Single, number);

            assert_eq!(throw, Err(InvalidThrowError::InvalidValue(number)));
        }
    }

    #[test]
    fn score_is_calculated_correctly() {
        let score = Throw::new(Multiplier::Triple, 20).unwrap().score();

        assert_eq!(score, 60);
    }
}
