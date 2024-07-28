use fixedbitset::FixedBitSet;

use crate::game::{
    board::{Board, GameState, BOARDSIZE},
    piece::PieceColor,
};

use super::{Eval, EvalInit};

pub const EDGE: u128 =   0b00000001111111111110000000001100000000011000000000110000000001100000000011000000000110000000001100000000011000000000111111111111;
pub const RING_1: u128 = 0b00000000000000000000000000000000000000000000000000000000100000000010100000000010000000000000000000000000000000000000000000000000;
pub const RING_2: u128 = 0b00000000000000000000000000000000000000000000010000000001010000000100010000000101000000000100000000000000000000000000000000000000;
pub const RING_3: u128 = 0b00000000000000000000000000000000001000000000101000000010001000001000001000001000100000001010000000001000000000000000000000000000;
pub const RING_4: u128 = 0b00000000000000000000000100000000010100000001000100000100000100010000000100010000010000010001000000010100000000010000000000000000;
pub const CORNER: u128 = 0b00000000010000010001000000010100000000010000000000000000000000000000000000000000000000000000000100000000010100000001000100000100;

pub struct HumanScoreParam {
    pub(crate) w_ring_1: f64,
    pub(crate) w_ring_2: f64,
    pub(crate) w_ring_3: f64,
    pub(crate) w_ring_4: f64,
    pub(crate) w_corner: f64,
    pub(crate) w_edge: f64,
    pub(crate) w_king_dst: f64,
}

pub struct HumanScore {
    w_ring_1: f64,
    w_ring_2: f64,
    w_ring_3: f64,
    w_ring_4: f64,
    w_corner: f64,
    w_edge: f64,
    w_king_dst: f64,
}

impl EvalInit for HumanScore {
    type Param = HumanScoreParam;

    fn new(param: Self::Param) -> Self {
        HumanScore {
            w_ring_1: param.w_ring_1,
            w_ring_2: param.w_ring_2,
            w_ring_3: param.w_ring_3,
            w_ring_4: param.w_ring_4,
            w_king_dst: param.w_king_dst,
            w_edge: param.w_edge,
            w_corner: param.w_corner,
        }
    }
}

impl Eval for HumanScore {
    fn get_eval(&self, board: &Board) -> f64 {
        match board.who_won() {
            GameState::WinAttacker => return 1000.0,
            GameState::WinDefender => return -1000.0,
            GameState::Draw => return 0.0,
            GameState::Undecided => {}
        };

        let black_on_ring_1 = (board.get_attacker() & RING_1).count_ones() as f64; //see appendix of written report
        let black_on_ring_2 = (board.get_attacker() & RING_2).count_ones() as f64;
        let black_on_ring_3 = (board.get_attacker() & RING_3).count_ones() as f64;
        let black_on_ring_4 = (board.get_attacker() & RING_4).count_ones() as f64;
        let black_on_corners = (board.get_attacker() & CORNER).count_ones() as f64;
        //let center_of_board = RING_1 | RING_2 | RING_3 | (1 << 60);

        let black_pos_sum = self.w_ring_1 * black_on_ring_1
            + self.w_ring_2 * black_on_ring_2
            + self.w_ring_3 * black_on_ring_3
            + self.w_ring_4 * black_on_ring_4
            + self.w_corner * black_on_corners;

        //bonus for blocking whites movement on columns and rows, controlling as many as possible
        let mut white_penalty = 0.0;
        for line_num in 0..BOARDSIZE {
            let mask: u128 = 2047 << (line_num);
            if (board.get_attacker() & mask).count_ones() != 0 {
                white_penalty += 1.0;
            };
        }

        for col_num in 0..BOARDSIZE {
            let mask: u128 = 1298708349570020393652962442872833 << (col_num);
            if (board.get_attacker() & mask).count_ones() != 0 {
                white_penalty += 1.0;
            };
        }

        if (board.get_king() & EDGE).count_ones() > 0 {
            white_penalty += self.w_edge * 1.0; //bonus for white if white is on edge with king
        }

        white_penalty +=
            self.w_king_dst * (10.0 - board.get_king_pos().unwrap().min_dist_to_corner() as f64);

        board.number_of_colored_pieces(&PieceColor::Attacker) as f64
            - 2.0 * board.number_of_colored_pieces(&PieceColor::Defender) as f64
            + black_pos_sum
            + white_penalty
    }

    fn update(&mut self, _board: Board) {}
}

#[allow(unused)]
fn fixedbitset_from_bitstring(bitstring: &str) -> FixedBitSet {
    let mut bitset = FixedBitSet::with_capacity(bitstring.len());
    for (i, bit) in bitstring.chars().rev().enumerate() {
        if bit == '1' {
            bitset.insert(i);
        }
    }
    bitset
}
