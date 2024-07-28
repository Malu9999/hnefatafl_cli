use std::{
    fs::File,
    io::{BufWriter, Write},
};

extern crate chrono;

use chrono::Local;
const MAX_NUMBER_OF_MOVES: usize = 1000;

use crate::{
    agent::Bot,
    game::{
        board::{Board, GameState},
        r#move::Move,
    },
};

pub struct Arena<'a> {
    black_bot: &'a mut Box<dyn Bot>,
    white_bot: &'a mut Box<dyn Bot>,
    replay_buffer: Vec<(Move, Board)>,
}

pub struct FightInfo {
    state: GameState,
    num_turns: usize,
    black_nodes: Vec<usize>,
    white_nodes: Vec<usize>,
}

impl Arena<'_> {
    pub fn new<'a>(black_bot: &'a mut Box<dyn Bot>, white_bot: &'a mut Box<dyn Bot>) -> Arena<'a> {
        Arena {
            black_bot,
            white_bot,
            replay_buffer: Vec::new(),
        }
    }
    //let the two bots of the arena fight with set time limits in ms
    pub fn fight_or_be_forgotten(
        &mut self,
        time_to_think_black: u128,
        time_to_think_white: u128,
        verbose: bool,
    ) -> FightInfo {
        let mut board = Board::new();
        let mut black_move = true;
        let mut num_of_turns = 0;
        self.replay_buffer = Vec::new();

        let mut num_black_nodes: Vec<usize> = Vec::with_capacity(MAX_NUMBER_OF_MOVES / 2);
        let mut num_white_nodes: Vec<usize> = Vec::with_capacity(MAX_NUMBER_OF_MOVES / 2);

        while !board.is_game_over() && num_of_turns < MAX_NUMBER_OF_MOVES {
            if verbose {
                println!("{}", board);
            }
            let mov = match black_move {
                true => {
                    let next_mov = self.black_bot.get_next_move(&board, time_to_think_black);
                    num_black_nodes.push(self.black_bot.num_nodes());
                    next_mov
                }
                false => {
                    let next_mov = self.white_bot.get_next_move(&board, time_to_think_white);
                    num_white_nodes.push(self.white_bot.num_nodes());
                    next_mov
                }
            };

            match mov {
                Some(mov) => {
                    if verbose {
                        println!("Doing move: {}", mov);
                    }
                    board.make_move_captured_positions(&mov);
                    self.replay_buffer.push((mov, board.clone()));
                }
                None => {
                    println!("could not make any move... Game over???");
                    break;
                }
            }

            black_move = !black_move;
            num_of_turns += 1;
        }

        FightInfo {
            state: board.who_won(),
            num_turns: num_of_turns,
            black_nodes: num_black_nodes,
            white_nodes: num_white_nodes,
        }
    }

    //let the two bots fight a number of times and write the replays and results into files
    pub fn play_games(
        &mut self,
        num_games: usize,
        time_to_think_black: u128,
        time_to_think_white: u128,
        verbose: bool,
    ) {
        let date = Local::now().format("%Y-%m-%d-%H-%M-%S").to_string();
        let file_name = format!(
            "{}-{}-{}-{}-{}-{}",
            date,
            self.black_bot.get_name(),
            time_to_think_black,
            self.white_bot.get_name(),
            time_to_think_white,
            num_games,
        );
        let ouput_results = File::create(format!("./results/{}", &file_name)).unwrap();

        let ouput_replays = File::create(format!("./replays/{}", &file_name)).unwrap();

        let mut writer_results = BufWriter::new(ouput_results);
        let mut writer_replay = BufWriter::new(ouput_replays);

        let _ = writer_results
            .write_all("Game_ID, Result, num_turns, num_avg_black_nodes, std_black_nodes, num_avg_white_nodes, std_white_nodes\n".as_bytes());

        let mut num_moves: Vec<usize> = Vec::with_capacity(num_games);
        let mut black_wins: usize = 0;

        for game_idx in 0..num_games {
            let current_fight_info =
                self.fight_or_be_forgotten(time_to_think_black, time_to_think_white, verbose);
            num_moves.push(current_fight_info.num_turns);

            if let GameState::WinAttacker = current_fight_info.state {
                black_wins += 1;
            }

            if verbose {
                match current_fight_info.state {
                    GameState::WinAttacker => println!("Black won!"),
                    GameState::WinDefender => println!("White won!"),
                    GameState::Draw => println!("It's a draw."),
                    GameState::Undecided => println!("Time ran out."),
                }
            }

            let _ = self.write_fight_info(&mut writer_results, current_fight_info, game_idx);
            let _ = self.write_replay(&mut writer_replay, game_idx);
        }

        let (mean, var) = mean_and_variance(&num_moves);
        let black_wr = black_wins as f64 / num_games as f64;
        let _ = writer_results
            .write_all(format!("{:.2}, {:.2}, {:.2}", mean, var, black_wr).as_bytes());
        println!("Finished");
    }

    //title encoding: <time> <black_bot_type> <black_time_to_think> <white_bot_type> <white_time_to_think> <number_of_games_played>
    //last line in result file: <avg num of turns> <standard deviation of number of turns> <black winrate>
    fn write_fight_info(
        &self,
        output: &mut BufWriter<File>,
        info: FightInfo,
        game_idx: usize,
    ) -> Result<(), std::io::Error> {
        let game_result = match info.state {
            GameState::WinAttacker => 1,
            GameState::WinDefender => -1,
            GameState::Draw => 0,
            GameState::Undecided => 2,
        };

        let (black_mean, black_var) = mean_and_variance(&info.black_nodes);
        let (white_mean, white_var) = mean_and_variance(&info.white_nodes);

        output.write_all(
            format!(
                "{}, {}, {}, {:.02}, {:.02}, {:.02}, {:.02}\n",
                game_idx, game_result, info.num_turns, black_mean, black_var, white_mean, white_var
            )
            .as_bytes(),
        )?;
        let _ = output.flush();
        Ok(())
    }

    fn write_replay(
        &self,
        output: &mut BufWriter<File>,
        game_idx: usize,
    ) -> Result<(), std::io::Error> {
        output.write_all(format!("Game: {}\n", game_idx).as_bytes())?;
        for (mov, _) in self.get_replay_buffer() {
            output.write_all(format!("{}\n", mov).as_bytes())?;
            // output.write_all(format!("{}", brd).as_bytes())?;
        }
        let _ = output.flush();
        Ok(())
    }

    pub fn get_replay_buffer(&self) -> Vec<(Move, Board)> {
        self.replay_buffer.clone()
    }
}

fn mean_and_variance(data: &Vec<usize>) -> (f64, f64) {
    let mut mean_sum = 0;
    let n = data.len() as f64;
    if n == 0.0 {
        return (-1.0, -1.0);
    }
    for point in data {
        mean_sum += point;
    }
    let mean = (mean_sum as f64) / n;

    let mut variance_sum = 0.0;

    for point in data {
        variance_sum += (*point as f64 - mean) * (*point as f64 - mean);
    }

    let variance = variance_sum / n;

    (mean, variance.sqrt())
}
