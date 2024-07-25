use std::str::SplitWhitespace;

use fixedbitset::FixedBitSet;
use serde::{Deserialize, Serialize};

use crate::Action;

use super::{
    board::BOARDSIZE,
    position::{ParsePositionError, Position},
};

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

    pub fn new(start_pos: Position, end_pos: Position) -> Move {
        Move { start_pos, end_pos }
    }

    pub fn get_start_pos(&self) -> &Position {
        &self.start_pos
    }

    pub fn get_end_pos(&self) -> &Position {
        &self.end_pos
    }

    pub fn from_id(id: usize) -> Move {
        let start_pos = Position::new_n(id / 121);
        let end_pos = Position::new_n(id % 121);

        Move::new(start_pos, end_pos)
    }

    pub fn to_id(&self) -> usize {
        self.get_start_pos().get_num() * 121 + self.get_end_pos().get_num()
    }

    pub fn to_action(&self) -> Action {
        Action {
            id: self.to_id(),
            start_pos: self.get_start_pos().to_string(),
            end_pos: self.get_end_pos().to_string(),
        }
    }

    pub fn get_mask(&self) -> FixedBitSet {
        let mut mask = FixedBitSet::with_capacity(BOARDSIZE * BOARDSIZE);
        mask.insert(self.get_start_pos().get_num());
        mask.insert(self.get_end_pos().get_num());
        mask
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
