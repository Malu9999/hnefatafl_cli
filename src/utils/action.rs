use crate::game::{position::Position, r#move::Move};

pub enum Action {
    PossibleMoves(Position),
    MakeMove(Move),
    Quit,
    Nothing,
}

impl Action {
    /// Parse an action from a string
    pub fn from_str(str: String) -> Option<Action> {
        let mut parts = str.split_whitespace();

        let preamble = parts.next();

        // from string methods need exceptions and everything, but I'm too lazy now...
        match preamble {
            Some("pm") => {
                let suffix: Vec<&str> = parts.collect();
                if suffix.len() != 2 {
                    return None;
                }
                let new_pos = Position::from_str(&suffix);
                match new_pos {
                    Ok(pos) => Some(Action::PossibleMoves(pos)),
                    Err(err) => {
                        println!("Position could not be parsed: {}", err);
                        None
                    }
                }
            }
            Some("mm") => {
                let new_move = Move::from_str(parts);
                match new_move {
                    Ok(mov) => Some(Action::MakeMove(mov)),
                    Err(err) => {
                        println!("Move could not be parsed: {}", err);
                        None
                    }
                }
            }
            Some("quit") => Some(Action::Quit),
            None => Some(Action::Nothing),
            _ => None,
        }
    }
}
