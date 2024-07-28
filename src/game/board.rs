use std::fmt::Display;

use fixedbitset::FixedBitSet;
use tch::{Device, Tensor};

use crate::utils::action::Action;

use super::{
    piece::{Piece, PieceColor},
    position::Position,
    r#move::Move,
};
use rand::prelude::SliceRandom;

pub enum GameState {
    Undecided,
    WinAttacker,
    WinDefender,
    Draw,
}

#[derive(Clone)]
pub struct Board {
    attackers: u128,
    defenders: u128,
    king: u128,
    attacker_moves: Vec<Move>,
    defender_moves: Vec<Move>,
    player: PieceColor,
}

pub const BOARDSIZE: usize = 11;

impl Board {
    pub fn new() -> Self {
        let mut attackers: u128 = 0b00000000001111100000000100000000000000001000000000110000000001110000000111000000000110000000001000000000000000010000000011111000;
        let mut defenders: u128 = 0b00000000000000000000000000000000000000000000010000000001110000000110110000000111000000000100000000000000000000000000000000000000;
        let mut king: u128 = 0b00000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000;

        let mut board = Board {
            attackers,
            defenders,
            king,
            attacker_moves: vec![],
            defender_moves: vec![],
            player: PieceColor::Attacker,
        };

        board.update_possible_moves();

        board
    }

    pub fn get_random_move(&self) -> Option<Move> {
        self.get_random_move_color(&self.player)
    }

    /// Returns a random possible move for the provided color
    pub fn get_random_move_color(&self, color: &PieceColor) -> Option<Move> {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        match color {
            PieceColor::Attacker => self.attacker_moves.choose(&mut rng).cloned(),
            PieceColor::Defender => self.defender_moves.choose(&mut rng).cloned(),
        }
    }

    pub fn get_legal_moves(&self) -> Vec<Move> {
        self.get_moves_color(&self.player)
    }

    /// Returns a copy of the currently possible moves
    pub fn get_moves_color(&self, color: &PieceColor) -> Vec<Move> {
        match color {
            PieceColor::Attacker => self.attacker_moves.clone(),
            PieceColor::Defender => self.defender_moves.clone(),
        }
    }

    fn get_piece(&self, pos: &Position) -> Option<Piece> {
        let pos_num: usize = pos.get_num();

        if (self.attackers >> pos_num) & 1 == 1 {
            return Some(Piece::Pawn(PieceColor::Attacker));
        } else if (self.defenders >> pos_num) & 1 == 1 {
            return Some(Piece::Pawn(PieceColor::Defender));
        } else if (self.king >> pos_num) & 1 == 1 {
            return Some(Piece::King(PieceColor::Defender));
        }
        None
    }

    pub fn has_color_piece(&self, pos: &Position, color: &PieceColor) -> bool {
        let pos_num: usize = pos.get_num();

        match color {
            PieceColor::Attacker => (self.attackers >> pos_num) & 1 == 1,
            PieceColor::Defender => {
                (self.defenders >> pos_num) & 1 == 1 || (self.king >> pos_num) & 1 == 1
            }
        }
    }

    pub fn get_king_pos(&self) -> Option<Position> {
        if self.king == 0 {
            None
        } else {
            Some(Position::new_n(self.king.ilog2() as usize))
        }
    }

    /// resturn one-hot encoding of the current state
    pub fn get_observation(&self) -> Tensor {
        let attack_vec: Vec<f32> = (0..BOARDSIZE * BOARDSIZE)
            .map(|i| {
                if self.attackers >> i & 1 == 1 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let defend_vec: Vec<f32> = (0..BOARDSIZE * BOARDSIZE)
            .map(|i| {
                if self.defenders >> i & 1 == 1 {
                    1.0
                } else {
                    0.0
                }
            })
            .collect();
        let king_vec: Vec<f32> = (0..BOARDSIZE * BOARDSIZE)
            .map(|i| if self.king >> i & 1 == 1 { 1.0 } else { 0.0 })
            .collect();
        let player_vec: Vec<f32> = (0..BOARDSIZE * BOARDSIZE)
            .map(|_| match self.player {
                PieceColor::Attacker => -1.0,
                PieceColor::Defender => 1.0,
            })
            .collect();

        let attack = Tensor::from_slice(&attack_vec);
        let defend = Tensor::from_slice(&defend_vec);
        let king = Tensor::from_slice(&king_vec);
        let player = Tensor::from_slice(&player_vec);

        Tensor::stack(&[attack, defend, king, player], 0)
    }

    fn remove_color_piece(&mut self, pos: &Position, color: &PieceColor) {
        let pos_m = pos.get_pos_mask();
        match color {
            PieceColor::Attacker => self.attackers ^= pos_m,
            PieceColor::Defender => {
                if self.get_king_pos().is_some_and(|p| p == *pos) {
                    self.king ^= pos_m;
                } else {
                    self.defenders ^= pos_m;
                }
            }
        }
    }

    pub fn pos_is_occupied(&self, pos: &Position) -> bool {
        self.has_color_piece(pos, &PieceColor::Attacker)
            || self.has_color_piece(pos, &PieceColor::Defender)
    }

    pub fn update_possible_moves(&mut self) {
        self.attacker_moves = self.possible_moves_color(&PieceColor::Attacker);
        self.defender_moves = self.possible_moves_color(&PieceColor::Defender);
    }

    /// Computes the possible moves from state self for the given color
    pub fn possible_moves_color(&self, color: &PieceColor) -> Vec<Move> {
        let mut possible_moves = Vec::<Move>::with_capacity(120);

        let mut current = match color {
            PieceColor::Attacker => &self.attackers,
            PieceColor::Defender => {
                if let Some(king_pos) = self.get_king_pos() {
                    possible_moves.extend(self.possible_moves_from_pos(&king_pos).unwrap());
                }
                &self.defenders
            }
        }
        .clone();

        for idx in 0..BOARDSIZE * BOARDSIZE {
            if current & 1 == 1 {
                let current_pos = Position::new_n(idx);
                possible_moves.extend(self.possible_moves_from_pos(&current_pos).unwrap());
            }
            current >>= 1;
        }

        possible_moves
    }

    pub fn possible_moves_from_pos(&self, pos: &Position) -> Option<Vec<Move>> {
        let current_piece = self.get_piece(pos)?;

        let start_x = pos.get_x();
        let start_y = pos.get_y();

        let mut possible_moves = Vec::<Move>::with_capacity(40);

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
        let mask = mov.get_mask();
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

        self.player.flip();

        captured_positions
    }

    /// computes the current state of the game
    pub fn who_won(&self) -> GameState {
        let attacker_moves_cnt = self.attacker_moves.len();
        let defender_moves_cnt = self.defender_moves.len();
        if self.get_king_pos().is_none() || defender_moves_cnt == 0 {
            GameState::WinAttacker
        } else if self.get_king_pos().unwrap().is_corner() || attacker_moves_cnt == 0 {
            GameState::WinDefender
        } else if attacker_moves_cnt == 0 || defender_moves_cnt == 0 {
            GameState::Draw
        } else {
            GameState::Undecided
        }
    }

    //returns color of piece on position
    fn get_color_of_pos(&self, pos: &Position) -> PieceColor {
        match (
            self.has_color_piece(pos, &PieceColor::Attacker),
            self.has_color_piece(pos, &PieceColor::Defender),
        ) {
            (true, false) => PieceColor::Attacker,
            (false, true) => PieceColor::Defender,
            (false, false) => panic!("Position {} was empty.", pos),
            (true, true) => panic!("Position {} is occupied by both players.", pos),
        }
    }

    // Check if move is valid
    pub fn is_valid_move(&self, mov: &Move, color: &PieceColor) -> bool {
        match color {
            PieceColor::Attacker => self.attacker_moves.contains(mov),
            PieceColor::Defender => self.defender_moves.contains(mov),
        }
    }

    pub fn perform_action(
        &mut self,
        action: &Action,
        player_color: &PieceColor,
    ) -> Result<(), &str> {
        match action {
            Action::MakeMove(mov) => {
                if self.is_valid_move(mov, player_color) {
                    if self.get_color_of_pos(mov.get_start_pos()) != *player_color {
                        return Err("move for other color.");
                    }
                    let _ = self.make_move_captured_positions(mov);
                } else {
                    return Err("move invalid.");
                }
            }
            Action::PossibleMoves(pos) => match self.possible_moves_from_pos(pos) {
                Some(moves) => {
                    println!(
                        "possible moves for {}  at {}: ",
                        self.get_piece(pos).unwrap(),
                        pos
                    );
                    for mov in moves {
                        print!("{}, ", mov);
                    }
                    println!();
                }
                None => println!("No piece at {}", &pos),
            },
            _ => (),
        }
        Ok(())
    }

    pub fn number_of_colored_pieces(&self, color: &PieceColor) -> u32 {
        match color {
            PieceColor::Attacker => self.attackers.count_ones(),
            PieceColor::Defender => self.defenders.count_ones(),
        }
    }

    pub fn is_game_over(&self) -> bool {
        !matches!(self.who_won(), GameState::Undecided)
    }

    pub fn get_player(&self) -> PieceColor {
        self.player.clone()
    }

    pub fn get_attacker(&self) -> &u128 {
        &self.attackers
    }

    pub fn get_defender(&self) -> &u128 {
        &self.defenders
    }

    pub fn get_king(&self) -> &u128 {
        &self.defenders
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
