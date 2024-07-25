mod game;

use clap::builder::Str;
use game::position::Position;
use game::r#move::Move;
use game::{board::Board, piece::PieceColor};

#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Action {
    id: usize, // Define according to your game
    start_pos: String,
    end_pos: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct State {
    winner: String,
    board: String,
    captures: Vec<Position>,
}

type Games = Arc<Mutex<HashMap<Uuid, Board>>>;

#[post("/new_game")]
fn new_game(games: &rocket::State<Games>) -> Json<Uuid> {
    let new_game_state = Board::init();
    let game_id = Uuid::new_v4();

    games.lock().unwrap().insert(game_id, new_game_state);

    Json(game_id)
}

#[post("/perform_action/<game_id>", data = "<action>")]
fn perform_action(
    game_id: String,
    action: Json<Action>,
    games: &rocket::State<Games>,
) -> Option<Json<State>> {
    let game_id = Uuid::parse_str(&game_id).ok()?;
    let mut games = games.lock().unwrap();

    if let Some(game) = games.get_mut(&game_id) {
        let mov = Move::from_id(action.id);

        let captured_pieces = game.make_move_captured_positions(&mov);
        let is_over = game.who_won();

        let state = State {
            winner: match is_over {
                game::board::GameState::Undecided => "Undecided".to_owned(),
                game::board::GameState::WinBlack => "Attacker".to_owned(),
                game::board::GameState::WinWhite => "Defender".to_owned(),
                game::board::GameState::Draw => "Draw".to_owned(),
            },
            board: game.to_string(),
            captures: captured_pieces,
        };

        Some(Json(state))
    } else {
        None
    }
}

#[get("/legal_moves/<game_id>")]
fn get_legal_moves(game_id: String, games: &rocket::State<Games>) -> Option<Json<Vec<Action>>> {
    let game_id = Uuid::parse_str(&game_id).ok()?;
    let games = games.lock().unwrap();

    if let Some(game) = games.get(&game_id) {
        // Calculate legal moves based on game state
        let legal_moves = game
            .get_legal_moves()
            .iter()
            .map(|mv| mv.to_action())
            .collect(); // Replace with actual legal moves logic
        Some(Json(legal_moves))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    let games: Games = Arc::new(Mutex::new(HashMap::new()));
    rocket::build()
        .manage(games)
        .mount("/", routes![new_game, perform_action, get_legal_moves])
}
