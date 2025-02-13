use std::num::ParseIntError;

use super::board::BOARDSIZE;

#[derive(Debug)]
pub struct ParsePositionError {
    kind: PositionErrorKind,
}

#[derive(Debug)]
enum PositionErrorKind {
    WrongDataAmount,
    OutOfRange,
    NonLetter,
    IntParsing(ParseIntError),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    num: usize,
}

impl Position {
    //position is encoded as u8 which represents the position of the bit in board_u128
    //functions present that can translate between (x,y) and u8 representation
    pub fn from_str(elements: &[&str]) -> Result<Position, ParsePositionError> {
        if elements.len() != 2 {
            return Err(ParsePositionError {
                kind: PositionErrorKind::WrongDataAmount,
            });
        }
        let int_parse = |el: &str| match el.parse::<usize>() {
            Ok(res) => Ok(res),
            Err(err) => Err(ParsePositionError {
                kind: PositionErrorKind::IntParsing(err),
            }),
        };

        let letter_parse = |el: &str| match el.chars().next() {
            Some(char) => Ok((char as u8 - b'A') as usize),
            None => Err(ParsePositionError {
                kind: PositionErrorKind::NonLetter,
            }),
        };

        let i: usize = int_parse(elements[0])?;
        let j: usize = letter_parse(elements[1])?;

        if i >= BOARDSIZE || j >= BOARDSIZE {
            return Err(ParsePositionError {
                kind: PositionErrorKind::OutOfRange,
            });
        }

        Ok(Position::new_xy(i, j))
    }

    /// Returns the manhatten distance
    pub fn manhatten_dist(&self, other: &Position) -> usize {
        self.get_x().abs_diff(other.get_x()) + self.get_y().abs_diff(other.get_y())
    }

    /// Returns the mask of the position
    pub fn get_pos_mask(&self) -> u128 {
        1 << self.num
    }

    /// Returns the manhatten distance to the closest corner
    pub fn min_dist_to_corner(&self) -> usize {
        [
            Position::new_n(0).manhatten_dist(self),
            Position::new_n(10).manhatten_dist(self),
            Position::new_n(110).manhatten_dist(self),
            Position::new_n(120).manhatten_dist(self),
        ]
        .into_iter()
        .min()
        .unwrap()
    }

    /// Create a position from x and y
    pub fn new_xy(x: usize, y: usize) -> Position {
        Position {
            num: x * BOARDSIZE + y,
        }
    }

    /// Create a position from a number
    pub fn new_n(num: usize) -> Position {
        Position { num }
    }

    /// Returns the x coordinate
    pub fn get_x(&self) -> usize {
        self.num / BOARDSIZE
    }

    /// Returns the y coordinate
    pub fn get_y(&self) -> usize {
        self.num % BOARDSIZE
    }

    /// Returns the number of the position
    pub fn get_num(&self) -> usize {
        self.num
    }

    /// Returns true if the position is a throne
    pub fn is_throne(&self) -> bool {
        [0, 10, 60, 110, 120].contains(&self.get_num())
    }

    /// Returns true if the position is a corner
    pub fn is_corner(&self) -> bool {
        [0, 10, 110, 120].contains(&self.get_num())
    }

    /// Returns a vector of (sur_pos, one_after)
    pub fn get_sur_pos_and_one_after(&self) -> Vec<(Position, Position)> {
        let mut sur_pos_and_one_after = Vec::<(Position, Position)>::new();

        let x = self.get_x();
        let y = self.get_y();

        // left
        if x >= 2 {
            sur_pos_and_one_after.push((
                Position::new_n(self.num - BOARDSIZE),
                Position::new_n(self.num - 2 * BOARDSIZE),
            ));
        }

        // right
        if x <= BOARDSIZE - 3 {
            sur_pos_and_one_after.push((
                Position::new_n(self.num + BOARDSIZE),
                Position::new_n(self.num + 2 * BOARDSIZE),
            ));
        }

        // up
        if y >= 2 {
            sur_pos_and_one_after
                .push((Position::new_n(self.num - 1), Position::new_n(self.num - 2)));
        }

        // down
        if y <= BOARDSIZE - 3 {
            sur_pos_and_one_after
                .push((Position::new_n(self.num + 1), Position::new_n(self.num + 2)));
        }

        sur_pos_and_one_after
    }

    /// Get all surrounding positions
    pub fn get_surrounding_pos(&self) -> Vec<Position> {
        let mut surrounding_pos = Vec::<Position>::new();

        let x = self.get_x();
        let y = self.get_y();

        // left
        if x >= 1 {
            surrounding_pos.push(Position::new_n(self.num - BOARDSIZE));
        }

        // right
        if x <= BOARDSIZE - 2 {
            surrounding_pos.push(Position::new_n(self.num + BOARDSIZE));
        }

        // up
        if y >= 1 {
            surrounding_pos.push(Position::new_n(self.num - 1));
        }

        // down
        if y <= BOARDSIZE - 2 {
            surrounding_pos.push(Position::new_n(self.num + 1));
        }

        surrounding_pos
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {})",
            self.get_x(),
            (65 + self.get_y() as u8) as char
        )
    }
}

impl std::fmt::Display for ParsePositionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PositionErrorKind::WrongDataAmount => write!(f, "wrong amount of data provided."),
            PositionErrorKind::IntParsing(err) => {
                write!(f, "Integers could not be parsed: {}.", err)
            }
            PositionErrorKind::OutOfRange => write!(f, "value is not in range."),
            PositionErrorKind::NonLetter => write!(
                f,
                "Second position index invalid. (Must be an UPPERCASE letter)"
            ),
        }
    }
}
