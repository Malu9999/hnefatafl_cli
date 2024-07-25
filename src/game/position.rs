use std::num::ParseIntError;

use super::board::BOARDSIZE;

#[derive(Debug)]
pub struct ParsePositionError {
    kind: PositionErrorKind,
}

#[derive(Debug)]
enum PositionErrorKind {
    WrongDataAmount,
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
        let m = |el: &str| match el.parse::<usize>() {
            Ok(res) => Ok(res),
            Err(err) => Err(ParsePositionError {
                kind: PositionErrorKind::IntParsing(err),
            }),
        };

        let i: usize = m(elements[0])?;
        let j: usize = m(elements[1])?;

        Ok(Position::new_xy(i, j))
    }

    pub fn manhatten_dist(&self, other: &Position) -> usize {
        self.get_x().abs_diff(other.get_x()) + self.get_y().abs_diff(other.get_y())
    }

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

    pub fn new_xy(x: usize, y: usize) -> Position {
        Position {
            num: x * BOARDSIZE + y,
        }
    }

    pub fn new_n(num: usize) -> Position {
        Position { num }
    }

    pub fn get_x(&self) -> usize {
        self.num / BOARDSIZE
    }

    pub fn get_y(&self) -> usize {
        self.num % BOARDSIZE
    }

    pub fn get_num(&self) -> usize {
        self.num
    }

    pub fn is_throne(&self) -> bool {
        [0, 10, 60, 110, 120].contains(&self.get_num())
    }

    pub fn is_corner(&self) -> bool {
        [0, 10, 110, 120].contains(&self.get_num())
    }

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
            PositionErrorKind::WrongDataAmount => write!(f, "wrong amount of data provided"),
            PositionErrorKind::IntParsing(err) => {
                write!(f, "Integers could not be parsed: {}", err)
            }
        }
    }
}
