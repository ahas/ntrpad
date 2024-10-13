use std::{
  fs::{self, File},
  io::Write,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use tauri::{Emitter, Manager, State};
use tauri_plugin_dialog::DialogExt;

pub mod menu;

#[derive(Default)]
struct AppStateInner {
  current_file_path: PathBuf,
  latest_contents: String,
}

impl AppStateInner {
  pub fn is_file_open(&self) -> bool {
    self.current_file_path.is_file()
  }
}

type AppState = Arc<Mutex<AppStateInner>>;

#[tauri::command]
fn new_file(state: State<'_, AppState>) {
  let mut state = state.lock().unwrap();
  state.current_file_path.clear();
  state.latest_contents.clear();
}

#[tauri::command]
fn open_file(handle: tauri::AppHandle, state: State<'_, AppState>) {
  let h = handle.clone();
  let state = state.inner().clone();

  // !bug: blocking_pick_file does not work on Linux
  handle.dialog().file().pick_file(move |file_path| {
    let Some(file_path) = file_path.and_then(|x| x.into_path().ok()) else {
      h.emit("file-opening-canceled", ()).unwrap();
      return;
    };
    let contents = fs::read_to_string(&file_path).unwrap();
    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();

    let mut state = state.lock().unwrap();
    state.current_file_path = file_path;
    state.latest_contents = contents.clone();

    h.emit("file-open", &contents).unwrap();
    h.emit("update-file-name", file_name).unwrap();
  });
}

#[tauri::command]
fn save(handle: tauri::AppHandle, state: State<'_, AppState>, contents: &str) {
  let contents = contents.to_owned();
  let is_first_save = !state.lock().unwrap().is_file_open();
  let h = handle.clone();
  let state = state.inner().clone();

  if is_first_save {
    // !bug: blocking_save_file does not work on Linux
    handle.dialog().file().add_filter("Text Files", &["txt"]).save_file(move |file_path| {
      let Some(file_path) = file_path.and_then(|x| x.into_path().ok()) else {
        h.emit("file-saving-canceled", ()).unwrap();
        return;
      };
      let file_name = file_path.file_name().unwrap().to_str().unwrap().to_owned();

      let mut file = File::create(&file_path).unwrap();
      file.write_all(contents.as_bytes()).unwrap();

      let mut state = state.lock().unwrap();
      state.latest_contents = contents.to_string();
      state.current_file_path = file_path;

      h.emit("update-file-name", file_name).unwrap();
      h.emit("file-saved", ()).unwrap();
    });
  } else {
    let mut state = state.lock().unwrap();
    state.latest_contents = contents.to_string();

    let mut file = File::create(&state.current_file_path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    h.emit("file-saved", ()).unwrap();
  }
}

#[tauri::command]
fn save_as(handle: tauri::AppHandle, contents: &str) {
  let contents = contents.to_owned();

  // !bug: blocking_save_file does not work on Linux
  handle.dialog().file().add_filter("Text Files", &["txt"]).save_file(move |file_path| {
    let Some(file_path) = file_path else { return };
    let Some(file_path) = file_path.as_path() else { return };

    let mut file = File::create(&file_path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
  });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .menu(menu::build)
    .setup(|app| {
      app.manage(Arc::new(Mutex::new(AppStateInner::default())));
      app.on_menu_event(menu::event::handler);

      Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![new_file, open_file, save, save_as])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
