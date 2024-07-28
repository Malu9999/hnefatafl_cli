mod agent;
mod eval;
mod game;
mod gym;
mod synthesis;
mod utils;

use core::time;
use std::str::FromStr;
use std::thread::{self, JoinHandle};
use std::usize;

use agent::{alpha_beta::policy::AlphaBetaBot, mcts::policy::Mcts, random::policy::RandomBot};
use agent::{Bot, BotInit};
use eval::human_score::{HumanScore, HumanScoreParam};
use eval::neural_net::NeuralNet;
use eval::random_rollout::RandomRollout;
use eval::EvalInit;
use game::board::BOARDSIZE;
use game::move_generation::MoveGen;
use game::position::Position;
use game::{
    board::{Board, GameState},
    piece::PieceColor,
};
use gym::fight::Arena;
use synthesis::network::Network;
use tch::{nn::VarStore, Device, Tensor};
use utils::action::Action;
use utils::gen_data::{self, Generator};

fn main() {
    println!("Welcome to Hnefatafl.\nPlease choose one of the following options:");
    println!("1) Bot vs. Bot");
    println!("2) Human vs. Bot");
    println!("3) Training");
    let mode = read_usize_in_range(1, 3);

    if mode == 1 {
        println!("Playing Bot vs. Bot");
        let (black_time_limit, mut black_bot) = choose_bot(PieceColor::Attacker);
        let (white_time_limit, mut white_bot) = choose_bot(PieceColor::Defender);

        let mut arena = Arena::new(&mut black_bot, &mut white_bot);

        println!("How many games do you want the bots to play?");
        let games_to_play = read_usize_in_range(1, usize::MAX);

        println!("Do you want to see every move and board in real time? (0: no, 1: yes)");
        let verbose_int = read_usize_in_range(0, 1);
        let verbose = verbose_int == 1;

        println!("Playing games...");
        println!("Please see ./results for the outcomes.");
        arena.play_games(games_to_play, black_time_limit, white_time_limit, verbose);
    } else if mode == 2 {
        println!("Playing Human vs. Bot");
        println!("Which side do you want to play? (0: black, 1: white)");
        let side = read_usize_in_range(0, 1);

        let my_color = if side == 0 {
            println!("You chose to play black.");
            PieceColor::Attacker
        } else if side == 1 {
            println!("You chose to play white.");
            PieceColor::Defender
        } else {
            panic!("You didn't choose a color");
        };

        let (bot_time_limit, mut bot) = choose_bot(my_color.get_opposite());

        game_loop(my_color, bot_time_limit, &mut bot);
    } else if mode == 3 {
        simple_taining_loop();
    } else {
        println!("You didn't choose a valid play mode.");
    }

    //move_gen();
    /*let mut board = Board::new();

    let mut turn = PieceColor::Attacker;

    //let mut vs = VarStore::new(Device::cuda_if_available());

    let mut net = Network::new();

    net.load("model.ot");
    //let (observations, targets) = Generator::new(100).generate();

    println!("done");

    let eval = HumanScore::new(HumanScoreParam {
        w_ring_1: 0.0,
        w_ring_2: 0.0,
        w_ring_3: 3.0,
        w_ring_4: 0.0,
        w_corner: 10.0,
        w_edge: -1.0,
        w_king_dst: -1.0,
    });

    let mut alphabeta = AlphaBetaBot::new(Some(&board), 3, eval);

    let mut mcts = Mcts::new(Some(&board), 2.0, RandomRollout::new(1));
    //let mut random = RandomBot::new(Some(&board), 2, RandomRollout::new(1));
    //let mut mcts = Mcts::new(Some(&board), 2.0, RandomRollout::new(1));

    //net.train(observations, targets, 1000, &vs);

    //let _ = vs.save("test.net");

    while !board.is_game_over() {
        //mcts.reset(&board);

        let mov = match turn {
            PieceColor::Attacker => alphabeta.get_next_move(&board, 100).unwrap(),
            PieceColor::Defender => alphabeta.get_next_move(&board, 1000).unwrap(),
        };

        //let mov = board.get_random_move().unwrap();
        println!("{}", mov);
        //mcts.print_root();

        let captured = board.make_move_captured_positions(&mov);

        let obs = board
            .get_observation()
            .unsqueeze(0)
            .to_device(Device::cuda_if_available());
        println!("{:?}", f32::try_from(net.forward(&obs)));

        //println!("{}", board.get_king_pos().unwrap().min_dist_to_corner());

        turn.flip();
        println!("{}", board);

        thread::sleep(time::Duration::from_millis(500));
    }*/
}

fn choose_bot(color: PieceColor) -> (u128, Box<dyn Bot>) {
    let eval = HumanScore::new(HumanScoreParam {
        w_ring_1: 0.0,
        w_ring_2: 0.0,
        w_ring_3: 1.0,
        w_ring_4: 0.0,
        w_corner: 1.0,
        w_edge: -1.0,
        w_king_dst: -1.0,
    });

    let word_for_color = match color {
        PieceColor::Attacker => "Attacker",
        PieceColor::Defender => "Defender",
    };
    println!("Please choose the Bot playing {}:", word_for_color);
    println!("1) Random");
    println!("2) MCTS");
    println!("3) Alpha-Beta");
    let black_bot_choice = read_usize_in_range(1, 3);

    let bot: Box<dyn Bot> = if black_bot_choice == 1 {
        Box::new(RandomBot::new(2, RandomRollout::new(1)))
    } else if black_bot_choice == 2 {
        Box::new(Mcts::new(1.4, RandomRollout::new(1)))
    } else {
        println!("Choose the max depth of the search");
        let max_depth = read_usize_in_range(1, 5);
        Box::new(AlphaBetaBot::new(max_depth, eval))
    };
    println!(
        "How much time in ms should {} have for each move?",
        word_for_color
    );
    let time_limit: u128 = read_usize_in_range(0, usize::MAX)
        .try_into()
        .expect("Failed converting time limit.");

    (time_limit, bot)
}

fn game_loop(player_color: PieceColor, bot_time_limit: u128, bot: &mut Box<dyn Bot>) {
    println!("Welcome to Hnefatafl! :D");
    println!("You can make a move by typing 'mm X1 Y1 X2 Y2'.");
    println!("You can also get a list of possible moves for a position by typing 'pm X Y'.");
    println!("The position 'A 3' must be given as '10 3'.");
    let mut board = Board::new();

    let mut turn = PieceColor::Attacker;

    while !board.is_game_over() {
        println!("{}", board);

        if turn == player_color {
            println!("please perform an action.");
            let action = read_string();
            match action {
                Some(Action::Quit) => break,
                Some(Action::Nothing) => println!("No action performed"),
                Some(act) => {
                    let res = board.perform_action(&act, &player_color);

                    if let Action::MakeMove(_) = act {
                        match res {
                            Err(msg) => println!("{}", msg),
                            Ok(()) => turn.flip(),
                        }
                    }
                }
                None => println!("action does not exist, try 'mm' or 'pm'"),
            }
        } else {
            let mov = bot.get_next_move(&board, bot_time_limit);
            match mov {
                Some(m) => {
                    println!("Bot move: {}", m);
                    let _ = board.make_move_captured_positions(&m);
                }
                None => println!("Bot failed to move"),
            };
            turn.flip()
        }
        println!();
    }

    match board.who_won() {
        GameState::WinAttacker => println!("Attacker won!"),
        GameState::WinDefender => println!("Defender won!"),
        GameState::Draw => println!("It's a draw."),
        GameState::Undecided => println!("Time ran out."),
    }
}

fn read_string() -> Option<utils::action::Action> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    Action::from_str(input)
}

fn read_usize_in_range(min: usize, max: usize) -> usize {
    let mut input = String::new();
    let mut done = false;
    let mut result = min;

    while !done {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if min <= num && num <= max {
                    done = true;
                    result = num
                } else {
                    println!("Input mut be between {} and {}", min, max);
                }
            }
            Err(err) => {
                println!("Could not parse input due to error: {}", err);
            }
        }
    }

    result
}

fn move_gen() {
    let move_gen = MoveGen::new();
    for sq in 0..(BOARDSIZE * BOARDSIZE) {
        println!(
            "magic: {} {}",
            sq,
            move_gen.gen_magics(&Position::new_n(sq), 22)
        )
    }
}

fn simple_taining_loop() {
    let net = Network::new();

    for i in 0..100 {
        net.save("old.ot");

        // gen training data
        let gen = Generator::new(32);
        let (observations, targets, _, _) =
            gen.generate(false, "old.ot".to_string(), "old.ot".to_string());

        net.train(observations, targets, 10);
    }
}

fn alpha_zero_loop() {
    let mut net = Network::new();

    for i in 0..100 {
        net.save("old.ot");

        // gen training data
        let gen = Generator::new(32);
        let (observations, targets, _, _) =
            gen.generate(false, "old.ot".to_string(), "old.ot".to_string());

        net.train(observations, targets, 10);

        // test agains new network
        net.save("new.ot");

        let (_, _, black_wins_old, white_wins_new) =
            gen.generate(true, "old.ot".to_string(), "new.ot".to_string());

        let (_, _, black_wins_new, white_wins_old) =
            gen.generate(true, "new.ot".to_string(), "old.ot".to_string());

        if black_wins_new > black_wins_old && white_wins_new > white_wins_old {
            println!("I'm better {}", i);
            net.load("old.ot");
        } else {
            println!("I'm worse {}", i);
        }
    }
}
