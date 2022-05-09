/// Represents the score multiplier region of a [Throw]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Multiplier {
    Single,
    Double,
    Triple,
}

impl Multiplier {
    /// Get the actual number to multiply the thrown number with
    fn factor(&self) -> u8 {
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
    /// Triple for bullseye are not valid
    BullseyeTriple,
    /// Valid numbers are 1-20 inclusive
    InvalidNumber(u8),
    // Error during parse
    Unparseable(String),
}

impl std::fmt::Display for InvalidThrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidThrowError::BullseyeTriple => writeln!(f, "Bullseye cannot be a triple"),
            InvalidThrowError::InvalidNumber(val) => writeln!(f, "Throw has invalid value {val}"),
            InvalidThrowError::Unparseable(text) => writeln!(f, "Could not parse {}", text),
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
    pub fn bullseye(multiplier: Multiplier) -> ThrowResult {
        match multiplier {
            Multiplier::Triple => Err(InvalidThrowError::BullseyeTriple),
            mult => Ok(Throw::Bullseye(mult)),
        }
    }

    /// Create a new number 1-20 throw.
    pub fn number(multiplier: Multiplier, number: u8) -> ThrowResult {
        match number {
            number if (1u8..21u8).contains(&number) => Ok(Throw::Number(multiplier, number)),
            number => Err(InvalidThrowError::InvalidNumber(number)),
        }
    }

    /// Create a new single hit of a number
    pub fn single(number: u8) -> ThrowResult {
        Self::number(Multiplier::Single, number)
    }

    /// Create a new double hit of a number
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
    pub fn miss() -> ThrowResult {
        Ok(Throw::Miss)
    }

    fn parse_multiplier(ch: &char) -> Option<Multiplier> {
        match ch {
            'd' | 'D' => Some(Multiplier::Double),
            't' | 'T' => Some(Multiplier::Triple),
            _ => None,
        }
    }

    pub fn from_str(text: &str) -> ThrowResult {
        let mut chars = text.chars().peekable();

        match chars.peek() {
            None => return Err(InvalidThrowError::Unparseable(text.into())),
            Some(ch) => {
                let opt_mult = Self::parse_multiplier(ch);

                let mult = match opt_mult {
                    Some(mult) => {
                        chars.next(); // gotta skip the multiplier char
                        mult
                    }
                    None => Multiplier::Single,
                };

                let number = chars.collect::<String>().parse::<u8>().ok();

                match number {
                    Some(n) if (1..21).contains(&n) => Ok(Throw::Number(mult, n)),
                    Some(n) if n == 25 => Throw::bullseye(mult),
                    Some(n) if n == 0 => Ok(Throw::Miss),
                    _ => Err(InvalidThrowError::Unparseable(text.into())),
                }
            }
        }
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

    #[test]
    fn test_parse_miss() {
        assert_eq!(Throw::miss(), Throw::from_str("0"));
    }

    #[test]
    fn test_parse_bullseye() {
        assert_eq!(Throw::bullseye(Multiplier::Single), Throw::from_str("25"));
    }

    #[test]
    fn test_parse_double_bullseye() {
        assert_eq!(Throw::bullseye(Multiplier::Double), Throw::from_str("D25"));
    }

    #[test]
    fn test_parse_singles() {
        for number in 1..=20 {
            assert_eq!(Throw::single(number), Throw::from_str(&number.to_string()));
        }
    }
    #[test]
    fn test_parse_doubles() {
        for number in 1..=20 {
            let string = "D".to_owned() + &number.to_string();
            assert_eq!(Throw::double(number), Throw::from_str(&string));
        }
    }
    #[test]
    fn test_parse_triples() {
        for number in 1..=20 {
            let string = "T".to_owned() + &number.to_string();
            assert_eq!(Throw::triple(number), Throw::from_str(&string));
        }
    }
}
