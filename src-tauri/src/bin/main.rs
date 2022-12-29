#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use chess_ai::command::chess1::chess1_solve;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![chess_ai::command::chess1::chess1_solve])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
