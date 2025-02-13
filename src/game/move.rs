use std::str::SplitWhitespace;

use super::position::{ParsePositionError, Position};

#[derive(Debug)]
pub struct ParseMoveError {
    kind: MoveErrorKind,
}

#[derive(Debug)]
enum MoveErrorKind {
    WrongDataAmount,
    PositionError(ParsePositionError),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    start_pos: Position,
    end_pos: Position,
}

impl Move {
    /// Create a move from a string
    pub fn from_str(split: SplitWhitespace<'_>) -> Result<Move, ParseMoveError> {
        let l: Vec<&str> = split.take(4).collect();
        if l.len() != 4 {
            return Err(ParseMoveError {
                kind: MoveErrorKind::WrongDataAmount,
            });
        }
        let start_pos = Position::from_str(&l[0..2]);
        let end_pos = Position::from_str(&l[2..4]);

        if let Err(err) = start_pos {
            return Err(ParseMoveError {
                kind: MoveErrorKind::PositionError(err),
            });
        }

        if let Err(err) = end_pos {
            return Err(ParseMoveError {
                kind: MoveErrorKind::PositionError(err),
            });
        }

        Ok(Move {
            start_pos: start_pos.unwrap(),
            end_pos: end_pos.unwrap(),
        })
    }

    /// Create a new move
    pub fn new(start_pos: Position, end_pos: Position) -> Move {
        Move { start_pos, end_pos }
    }

    /// Get the start  position
    pub fn get_start_pos(&self) -> &Position {
        &self.start_pos
    }

    /// Get the end position
    pub fn get_end_pos(&self) -> &Position {
        &self.end_pos
    }

    /// Get the mask for the move
    pub fn get_mask(&self) -> u128 {
        self.get_start_pos().get_pos_mask() | self.get_end_pos().get_pos_mask()
    }
    //returns positions on path of move
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start_pos, self.end_pos)
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start_pos, self.end_pos)
    }
}

impl std::fmt::Display for ParseMoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            MoveErrorKind::WrongDataAmount => write!(f, "wrong amount of data provided"),
            MoveErrorKind::PositionError(err) => write!(f, "position could not be parsed: {}", err),
        }
    }
}
