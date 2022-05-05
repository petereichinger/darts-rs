#[derive(Debug, PartialEq, Eq)]
pub enum NewPlayerError {
    InvalidName(String),
}

impl std::error::Error for NewPlayerError {
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
impl std::fmt::Display for NewPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NewPlayerError::InvalidName(name) => {
                writeln!(f, "'{}' is not a valid name for a player", name)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: &str) -> Result<Player, NewPlayerError> {
        let owned_name = String::from(name);
        if name.trim().is_empty() {
            Err(NewPlayerError::InvalidName(owned_name))
        } else {
            Ok(Player { name: owned_name })
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creating_player_works() {
        let player = Player::new("Anna");
        assert_eq!(
            Ok(Player {
                name: String::from("Anna")
            }),
            player
        );
    }

    #[test]
    fn creating_with_whitespace_name_does_not_work() {
        let whitespace_name = String::from("     ");

        let player = Player::new(&whitespace_name);

        assert_eq!(Err(NewPlayerError::InvalidName(whitespace_name)), player);
    }

    #[test]
    fn creating_with_empty_name_does_not_work() {
        let empty_name = String::from("");

        let player = Player::new(&empty_name);

        assert_eq!(Err(NewPlayerError::InvalidName(empty_name)), player);
    }
}
