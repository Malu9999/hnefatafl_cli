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

    pub fn new_xy(x: usize, y: usize) -> Position {
        Position {
            num: x * BOARDSIZE + y,
        }
    }

    pub fn new_n(num: usize) -> Position {
        Position { num }
    }

    pub fn new_n_u8(num: u8) -> Position {
        Position { num: num as usize }
    }

    pub fn get_x(&self) -> usize {
        (self.num / BOARDSIZE) as usize
    }

    pub fn get_y(&self) -> usize {
        (self.num % BOARDSIZE) as usize
    }

    pub fn get_num(&self) -> usize {
        self.num
    }

    pub fn is_equal(&self, other: &Position) -> bool {
        self.get_num() == other.get_num()
    }

    pub fn is_valid(&self) -> bool {
        self.get_num() < 121
    }

    pub fn on_line(&self, other: &Position) -> bool {
        self.get_x() == other.get_x() || self.get_y() == other.get_y()
    }

    #[allow(unused)]
    pub fn same_direction(&self, other1: &Position, other2: &Position) -> bool {
        (self.get_x() == other1.get_x() && other1.get_x() == other2.get_x())
            || (self.get_y() == other1.get_y() && other1.get_y() == other2.get_y())
    }

    pub fn is_throne(&self) -> bool {
        [0, 10, 60, 110, 120].contains(&self.get_num())
    }

    pub fn is_corner(&self) -> bool {
        [0, 10, 110, 120].contains(&self.get_num())
    }

    #[allow(unused)]
    pub fn is_edge(&self) -> bool {
        let x = self.get_x();
        let y = self.get_y();
        x == 0 || x == BOARDSIZE - 1 || y == 0 || y == BOARDSIZE - 1
    }

    pub fn get_sur_pos_and_one_after(&self) -> Vec<(Position, Position)> {
        let mut sur_pos_and_one_after = Vec::<(Position, Position)>::new();

        let x = self.get_x();
        let y = self.get_y();

        // left
        if x >= 2 {
            sur_pos_and_one_after.push((
                Position::new_n(self.get_num() - BOARDSIZE),
                Position::new_n(self.get_num() - 2 * BOARDSIZE),
            ));
        }

        // right
        if x <= BOARDSIZE - 3 {
            sur_pos_and_one_after.push((
                Position::new_n(self.get_num() + BOARDSIZE),
                Position::new_n(self.get_num() + 2 * BOARDSIZE),
            ));
        }

        // up
        if y >= 2 {
            sur_pos_and_one_after.push((
                Position::new_n(self.get_num() - 1),
                Position::new_n(self.get_num() - 2),
            ));
        }

        // down
        if y <= BOARDSIZE - 3 {
            sur_pos_and_one_after.push((
                Position::new_n(self.get_num() + 1),
                Position::new_n(self.get_num() + 2),
            ));
        }

        sur_pos_and_one_after
    }

    pub fn get_surrounding_pos(&self) -> Vec<Position> {
        let mut surrounding_pos = Vec::<Position>::new();

        let x = self.get_x();
        let y = self.get_y();

        // left
        if x >= 1 {
            surrounding_pos.push(Position::new_n(self.get_num() - BOARDSIZE));
        }

        // right
        if x <= BOARDSIZE - 2 {
            surrounding_pos.push(Position::new_n(self.get_num() + BOARDSIZE));
        }

        // up
        if y >= 1 {
            surrounding_pos.push(Position::new_n(self.get_num() - 1));
        }

        // down
        if y <= BOARDSIZE - 2 {
            surrounding_pos.push(Position::new_n(self.get_num() + 1));
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
