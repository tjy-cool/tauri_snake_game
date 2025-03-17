mod game_core;
use game_core::{GameState, Direction};
use std::sync::Mutex;

#[derive(Default)]
struct GameStateWrapper(Mutex<Option<GameState>>);

#[tauri::command]
fn init_game(state: tauri::State<GameStateWrapper>, width: i32, height: i32, cell_size: i32) -> GameState {
    let game = GameState::new(width, height, cell_size);
    *state.0.lock().unwrap() = Some(game.clone());
    game
}

#[tauri::command]
fn update_game(state: tauri::State<GameStateWrapper>) -> Option<GameState> {
    let mut game_state = state.0.lock().unwrap();
    if let Some(game) = game_state.as_mut() {
        game.update();
        Some(game.clone())
    } else {
        None
    }
}

#[tauri::command]
fn change_direction(state: tauri::State<GameStateWrapper>, direction: Direction) {
    if let Some(game) = state.0.lock().unwrap().as_mut() {
        game.change_direction(direction);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(GameStateWrapper::default())
        .invoke_handler(tauri::generate_handler![init_game, update_game, change_direction])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
