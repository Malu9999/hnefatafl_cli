use std::fmt::Display;

use fixedbitset::FixedBitSet;

use super::{
    piece::{Piece, PieceColor},
    position::Position,
    r#move::Move,
};
use rand::prelude::SliceRandom;

pub enum GameState {
    Undecided,
    WinBlack,
    WinWhite,
    Draw,
}

pub struct Board {
    attackers: FixedBitSet,
    defenders: FixedBitSet,
    king: FixedBitSet,
    attacker_moves: Vec<Move>,
    defender_moves: Vec<Move>,
}

pub const BOARDSIZE: usize = 11;

impl Board {
    pub fn init() -> Board {
        let mut attackers = FixedBitSet::with_capacity(BOARDSIZE * BOARDSIZE);
        let mut defenders = FixedBitSet::with_capacity(BOARDSIZE * BOARDSIZE);
        let mut king = FixedBitSet::with_capacity(BOARDSIZE * BOARDSIZE);

        let attackes_pos = vec![
            3, 4, 5, 6, 7, 16, 33, 43, 44, 54, 55, 56, 64, 65, 66, 76, 77, 87, 104, 113, 114, 115,
            116, 117,
        ];
        let defenders_pos = vec![38, 48, 49, 50, 58, 59, 61, 62, 70, 71, 72, 82];

        for attacker in attackes_pos {
            attackers.insert(attacker);
        }
        for defender in defenders_pos {
            defenders.insert(defender);
        }
        king.insert(60);

        let mut board = Board {
            attackers,
            defenders,
            king,
            attacker_moves: vec![],
            defender_moves: vec![],
        };

        board.update_possible_moves();

        board
    }

    /// Returns a random possible move for the provided color
    pub fn get_random_move_color(&self, color: &PieceColor) -> Option<Move> {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        match color {
            PieceColor::Attacker => self.attacker_moves.choose(&mut rng).cloned(),
            PieceColor::Defender => self.defender_moves.choose(&mut rng).cloned(),
        }
    }

    /// Returns a copy of the currently possible moves
    pub fn get_moves_color(&self, color: &PieceColor) -> Vec<Move> {
        match color {
            PieceColor::Attacker => self.attacker_moves.clone(),
            PieceColor::Defender => self.defender_moves.clone(),
        }
    }

    pub fn get_piece(&self, pos: &Position) -> Option<Piece> {
        let pos_num = pos.get_num() as usize;
        if self.attackers.contains(pos_num) {
            return Some(Piece::Pawn(PieceColor::Attacker));
        } else if self.defenders.contains(pos_num) {
            return Some(Piece::Pawn(PieceColor::Defender));
        } else if self.king.contains(pos_num) {
            return Some(Piece::King(PieceColor::Defender));
        }
        None
    }

    pub fn has_color_piece(&self, pos: &Position, color: &PieceColor) -> bool {
        let pos_num = pos.get_num();
        match color {
            PieceColor::Attacker => self.attackers.contains(pos_num),
            PieceColor::Defender => self.defenders.contains(pos_num) || self.king.contains(pos_num),
        }
    }

    pub fn get_king_pos(&self) -> Option<Position> {
        Some(Position::new_n(self.king.maximum()?))
    }

    fn remove_color_piece(&mut self, pos: &Position, color: &PieceColor) {
        let pos_num = pos.get_num();
        match color {
            PieceColor::Attacker => self.attackers.remove(pos_num),
            PieceColor::Defender => {
                if self.get_king_pos().is_some_and(|p| p == *pos) {
                    self.king.remove(pos_num);
                } else {
                    self.defenders.remove(pos_num);
                }
            }
        }
    }

    pub fn pos_is_occupied(&self, pos: &Position) -> bool {
        let pos_num = pos.get_num();
        self.attackers.contains(pos_num)
            || self.defenders.contains(pos_num)
            || self.king.contains(pos_num)
    }

    pub fn update_possible_moves(&mut self) {
        self.attacker_moves = self.possible_moves_color(&PieceColor::Attacker);
        self.defender_moves = self.possible_moves_color(&PieceColor::Defender);
    }

    /// Computes the possible moves from state self for the given color
    pub fn possible_moves_color(&self, color: &PieceColor) -> Vec<Move> {
        let mut possible_moves = Vec::<Move>::with_capacity(120);

        let current = match color {
            PieceColor::Attacker => &self.attackers,
            PieceColor::Defender => {
                if let Some(king_pos) = self.get_king_pos() {
                    possible_moves.extend(self.possible_moves_from_pos(&king_pos).unwrap());
                }
                &self.defenders
            }
        };

        for idx in current.ones() {
            let current_pos = Position::new_n(idx);
            possible_moves.extend(self.possible_moves_from_pos(&current_pos).unwrap());
        }

        possible_moves
    }

    pub fn possible_moves_from_pos(&self, pos: &Position) -> Option<Vec<Move>> {
        let current_piece = self.get_piece(pos)?;

        let start_x = pos.get_x();
        let start_y = pos.get_y();

        let mut possible_moves = Vec::<Move>::with_capacity(20);

        // down
        for nx in start_x + 1..BOARDSIZE {
            let new_pos = Position::new_xy(nx, start_y);
            if self.pos_is_occupied(&new_pos) {
                break;
            }
            if new_pos.is_throne() && !current_piece.is_king() {
                continue;
            }
            possible_moves.push(Move::new(pos.clone(), new_pos));
        }

        // up
        for nx in (0..start_x).rev() {
            let new_pos = Position::new_xy(nx, start_y);
            if self.pos_is_occupied(&new_pos) {
                break;
            }
            if new_pos.is_throne() && !current_piece.is_king() {
                continue;
            }
            possible_moves.push(Move::new(pos.clone(), new_pos));
        }

        // right
        for ny in start_y + 1..BOARDSIZE {
            let new_pos = Position::new_xy(start_x, ny);
            if self.pos_is_occupied(&new_pos) {
                break;
            }
            if new_pos.is_throne() && !current_piece.is_king() {
                continue;
            }
            possible_moves.push(Move::new(pos.clone(), new_pos));
        }

        // left
        for ny in (0..start_y).rev() {
            let new_pos = Position::new_xy(start_x, ny);
            if self.pos_is_occupied(&new_pos) {
                break;
            }
            if new_pos.is_throne() && !current_piece.is_king() {
                continue;
            }
            possible_moves.push(Move::new(pos.clone(), new_pos));
        }

        Some(possible_moves)
    }

    /// Perfoms the provided move on self and return the captured positions
    /// This function also keeps the possible move for our board up to date.
    pub fn make_move_captured_positions(&mut self, mov: &Move) -> Vec<Position> {
        // get information about the move
        let start_pos = mov.get_start_pos();
        let end_pos = mov.get_end_pos();
        let moving_piece = self.get_piece(start_pos).unwrap();

        let color = moving_piece.get_color();
        let enemy_color = color.get_opposite();

        // move the piece on the bit-board using XOR operator
        let mask: FixedBitSet = mov.get_mask();
        match color {
            PieceColor::Attacker => self.attackers ^= mask,
            PieceColor::Defender => {
                if moving_piece.is_king() {
                    self.king ^= mask;
                } else {
                    self.defenders ^= mask;
                }
            }
        }

        // make the capture checks and store captured positions
        let mut captured_positions = vec![];

        // we look two steps in each direction (if it is possible)
        for (sur_pos, one_after) in end_pos.get_sur_pos_and_one_after() {
            // if the adjacent position does not have an enemy piece -> continue
            if !self.has_color_piece(&sur_pos, &enemy_color) {
                continue;
            }

            // check whether the considered pos is the position of the king
            if self.get_king_pos().unwrap() == sur_pos {
                // to capture a king, it must be completely surrounded by pieces (or thrones)
                if sur_pos
                    .get_surrounding_pos()
                    .iter()
                    .all(|king_sur| self.has_color_piece(king_sur, &color) || king_sur.is_throne())
                {
                    self.remove_color_piece(&sur_pos, &enemy_color);
                    captured_positions.push(sur_pos);

                    // need to break if king was removed - game over anyway
                    break;
                }
            } else {
                // to capture a normal pieve we just have to check the position one after
                // and capture if this is another of our pieces or a throne.
                if self.has_color_piece(&sur_pos, &enemy_color)
                    && (self.has_color_piece(&one_after, &color) || one_after.is_throne())
                {
                    self.remove_color_piece(&sur_pos, &enemy_color);
                    captured_positions.push(sur_pos);
                }
            }
        }

        // update the possible moves for our board
        self.update_possible_moves();

        captured_positions
    }

    /// computes the current state of the game
    pub fn who_won(&self) -> GameState {
        let attacker_moves_cnt = self.attacker_moves.len();
        let defender_moves_cnt = self.defender_moves.len();
        if self.get_king_pos().is_none() || defender_moves_cnt == 0 {
            println!("Black");
            GameState::WinBlack
        } else if self.get_king_pos().unwrap().is_corner() || attacker_moves_cnt == 0 {
            println!("White");
            GameState::WinWhite
        } else if attacker_moves_cnt == 0 || defender_moves_cnt == 0 {
            println!("Draw");
            GameState::Draw
        } else {
            GameState::Undecided
        }
    }

    pub fn is_game_over(&self) -> bool {
        !matches!(self.who_won(), GameState::Undecided)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "00 ")?;
        for i in 0..BOARDSIZE {
            write!(f, "{} ", (65 + i as u8) as char)?;
        }
        writeln!(f)?;
        for row_idx in 0..BOARDSIZE {
            write!(f, "{:02} ", row_idx)?;
            for col in 0..BOARDSIZE {
                let current_pos = Position::new_xy(row_idx, col);

                match self.get_piece(&current_pos) {
                    Some(piece) => write!(f, "{} ", piece),
                    None => write!(f, ". "),
                }?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
