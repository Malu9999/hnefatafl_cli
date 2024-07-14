use fixedbitset::FixedBitSet;

use crate::game::board::{Board, GameState};

use super::Eval;

pub struct HumanScore {
    w_ring_1: f64,
    w_ring_2: f64,
    w_ring_3: f64,
    w_ring_4: f64,
    w_corner: f64,
    w_edge: f64,
    w_king_dst: f64,
    ring_1: FixedBitSet,
    ring_2: FixedBitSet,
    ring_3: FixedBitSet,
    ring_4: FixedBitSet,
    corner: FixedBitSet,
    edge: FixedBitSet,
}

impl Eval for HumanScore {
    fn init() -> Self {
        HumanScore {
            w_ring_1: 1.0,
            w_ring_2: 1.0,
            w_ring_3: 1.0,
            w_ring_4: 1.0,
            w_king_dst: 10.0,
            w_edge: 1.0,
            w_corner: 1.0,
            ring_1: fixedbitset_from_bitstring("0000000000000000000000000000000000000000000000001000000000101000000000100000000000000000000000000000000000000000000000000"),
            ring_2: fixedbitset_from_bitstring("0000000000000000000000000000000000000010000000001010000000100010000000101000000000100000000000000000000000000000000000000"),
            ring_3: fixedbitset_from_bitstring("0000000000000000000000000001000000000101000000010001000001000001000001000100000001010000000001000000000000000000000000000"),
            ring_4: fixedbitset_from_bitstring("0000000000000000100000000010100000001000100000100000100010000000100010000010000010001000000010100000000010000000000000000"),
            corner: fixedbitset_from_bitstring("0010000010001000000010100000000010000000000000000000000000000000000000000000000000000000100000000010100000001000100000100"),
            edge: fixedbitset_from_bitstring("1111111111110000000001100000000011000000000110000000001100000000011000000000110000000001100000000011000000000111111111111"),
        }
    }

    fn get_eval(&self, board: &Board) -> f64 {
        match board.who_won() {
            GameState::WinAttacker => return 1000.0,
            GameState::WinDefender => return -1000.0,
            GameState::Draw => return 0.0,
            GameState::Undecided => {}
        };

        let black_on_ring_1 = self.ring_1.intersection_count(board.get_attacker()) as f64;
        let black_on_ring_2 = self.ring_2.intersection_count(board.get_attacker()) as f64;
        let black_on_ring_3 = self.ring_3.intersection_count(board.get_attacker()) as f64;
        let black_on_ring_4 = self.ring_4.intersection_count(board.get_attacker()) as f64;
        let black_on_corners = self.corner.intersection_count(board.get_attacker()) as f64;

        let winner = match board.who_won() {
            GameState::WinAttacker => 1000.0,
            GameState::WinDefender => -1000.0,
            _ => 0.0,
        };

        self.w_ring_1 * black_on_ring_1
            + self.w_ring_2 * black_on_ring_2
            + self.w_ring_3 * black_on_ring_3
            + self.w_ring_4 * black_on_ring_4
            + self.w_corner * black_on_corners
            + winner
            + self.w_king_dst * board.get_king_pos().unwrap().min_dist_to_corner() as f64
    }

    fn update(&mut self, board: Board) {
        todo!()
    }
}

fn fixedbitset_from_bitstring(bitstring: &str) -> FixedBitSet {
    let mut bitset = FixedBitSet::with_capacity(bitstring.len());
    for (i, bit) in bitstring.chars().rev().enumerate() {
        if bit == '1' {
            bitset.insert(i);
        }
    }
    bitset
}
