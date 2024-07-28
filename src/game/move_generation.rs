use rand::{rngs::ThreadRng, Rng};

use super::{
    board::{Board, BOARDSIZE},
    position::Position,
    r#move::Move,
};

// MASK for column, row and throne
const COLUMN: u128 =    0b00000000000000000100000000001000000000010000000000100000000001000000000010000000000100000000001000000000010000000000100000000001;
const ROW: u128 =       0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011111111111;
const THRONE: u128 =    0b00000001000000000100000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000010000000001;

#[allow(unused)]
pub struct MoveGen {
    magics: Vec<u128>,
    shifts: Vec<usize>,
    lookup: Vec<Vec<u128>>,
}

#[allow(unused)]
impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen {
            magics: vec![],
            shifts: vec![],
            lookup: vec![],
        }
    }

    /// Generate the mask for the sliders
    pub fn slider_mask(&self, pos: &Position) -> u128 {
        let (pos_x, pos_y) = (pos.get_x(), pos.get_y());

        (ROW << (pos_x * BOARDSIZE)) ^ (COLUMN << pos_y)
    }

    /// Generate all possible moves for a given position
    pub fn generate_moves(&self, pos: Position, board: &Board) -> Vec<Move> {
        let pos_num = pos.get_num();

        let all_pieces = board.get_attacker() | board.get_defender() | board.get_king();
        let blockers = all_pieces & self.slider_mask(&pos);
        let key = ((blockers * self.magics[pos_num]) >> self.shifts[pos_num]) as usize;

        let mut moves_bitboard = self.lookup[pos_num][key];

        if !board.get_king_pos().is_some_and(|k_pos| pos == k_pos) {
            moves_bitboard &= !THRONE;
        }

        // put moves into vector
        let mut moves = Vec::<Move>::new();
        let mut idx = 0;

        while moves_bitboard != 0 {
            if moves_bitboard & 1 == 1 {
                moves.push(Move::new(pos.clone(), Position::new_n(idx)));
            }
            moves_bitboard >>= 1;
            idx += 1;
        }

        moves
    }

    /// Generate all possible moves for a given position
    pub fn old_gen_moves(&self, pos: &Position, occupied: u128) -> u128 {
        let mut result: u128 = 0;

        let (start_x, start_y) = (pos.get_x(), pos.get_y());

        let get_num = |x, y| BOARDSIZE * x + y;

        // down
        for nx in start_x + 1..BOARDSIZE {
            if occupied >> get_num(nx, start_y) & 1 == 1 {
                break;
            }
            result |= 1 << get_num(nx, start_y);
        }

        // up
        for nx in (0..start_x).rev() {
            if occupied >> get_num(nx, start_y) & 1 == 1 {
                break;
            }
            result |= 1 << get_num(nx, start_y);
        }

        // right
        for ny in start_y + 1..BOARDSIZE {
            if occupied >> get_num(start_x, ny) & 1 == 1 {
                break;
            }
            result |= 1 << get_num(start_x, ny);
        }

        // left
        for ny in (0..start_y).rev() {
            if occupied >> get_num(start_x, ny) & 1 == 1 {
                break;
            }
            result |= 1 << get_num(start_x, ny);
        }

        result
    }

    /// Generate occupancy for a given index
    pub fn gen_occupied(&self, index: usize, num_fields: u32, mask: u128) -> u128 {
        let mut occupancy: u128 = 0;
        let mut mask_copy = mask;

        // works by moving the digits of the binary representation of the index
        // into the mask
        for idx in 0..num_fields {
            let b = (mask_copy ^ (mask_copy - 1)) & mask_copy;

            let first_one = b.ilog2();

            if (index >> idx) & 1 == 1 {
                occupancy |= 1 << first_one;
            }

            mask_copy ^= b;
        }

        occupancy
    }

    /// Transform a blocking to a hash index
    pub fn transform(&self, blocking: u128, magic: u128, bits: usize) -> usize {
        (((blocking * magic) & 0x1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) >> (121 - bits)) as usize
    }

    /// Generate a random u128 with few set bits
    pub fn random_few_bits(&self, rng: &mut ThreadRng) -> u128 {
        let u1: u128 = rng.gen();
        let u2: u128 = rng.gen();
        let u3: u128 = rng.gen();

        u1 & u2 & u3 & 0x1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
    }

    /// Generate magic numbers
    pub fn gen_magics(&self, pos: &Position, bits: usize) -> u128 {
        let mut rng = rand::thread_rng();

        // get information about the current position
        let mask = self.slider_mask(pos);
        let num_fields = mask.count_ones();

        print_board(mask);

        // compute correct moves for each blocking state
        let mut blockings = Vec::<u128>::with_capacity(1 << num_fields);
        let mut correct_move = Vec::<u128>::with_capacity(1 << num_fields);

        // fill the correct moves and blocking states
        for i in 0..(1 << num_fields) {
            let current_blocking = self.gen_occupied(i, num_fields, mask);
            blockings.push(current_blocking);
            correct_move.push(self.old_gen_moves(pos, current_blocking));
        }

        let mut flag: Vec<bool> = vec![false; 1 << bits];
        let mut used: Vec<u128> = vec![0; 1 << bits];
        let mut fail = false;

        // try to find a magic number by randomly generating magic numbers
        for it in 0..500_000 {
            let magic: u128 = self.random_few_bits(&mut rng);

            if (((mask * magic) & 0x1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF) >> (121 - bits)).count_ones()
                < 6
            {
                continue;
            }

            if it % 10_000 == 0 {
                println!("{}", it);
            }

            for i in 0..1 << bits {
                flag[i] = false;
                used[i] = 0;
            }

            // check if the magic number works
            for i in 0..(1 << num_fields) {
                let hash_idx = self.transform(blockings[i], magic, bits);
                if !flag[hash_idx] {
                    flag[hash_idx] = true;
                    used[hash_idx] = correct_move[i];
                } else if used[hash_idx] != correct_move[i] {
                    //println!("failed in it: {} of {}", i, 1 << num_fields);
                    fail = true;
                    break;
                }
            }

            if !fail {
                return magic;
            }
            fail = false;
        }

        println!("failed");

        0
    }
}

/// Print a bitboard for debug reasons
fn print_board(board: u128) {
    print!("00 ");
    for i in 0..BOARDSIZE {
        print!("{} ", (65 + i as u8) as char);
    }
    println!();
    for row_idx in 0..BOARDSIZE {
        print!("{:02} ", row_idx);
        for col in 0..BOARDSIZE {
            let current_pos = Position::new_xy(row_idx, col);

            if board >> (current_pos.get_num()) & 1 == 1 {
                print!("X ");
            } else {
                print!(". ")
            }
        }
        println!();
    }
}
