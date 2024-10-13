use tauri::{menu::MenuEvent, AppHandle, Emitter};

pub fn handler(handle: &AppHandle, event: MenuEvent) {
  let event_key = event.id().0.as_str();

  match event_key {
    "new-file" => handle.emit("invoke:new_file", ()).unwrap(),
    "open-file" => handle.emit("invoke:open_file", ()).unwrap(),
    "save" => handle.emit("invoke:save", ()).unwrap(),
    "save-as" => handle.emit("invoke:save_as", ()).unwrap(),
    "undo" => handle.emit("undo", ()).unwrap(),
    "redo" => handle.emit("redo", ()).unwrap(),
    "close-window" => handle.exit(0),
    "zoom-in" => handle.emit("zoom-in", ()).unwrap(),
    "zoom-out" => handle.emit("zoom-out", ()).unwrap(),
    "reset-zoom" => handle.emit("reset-zoom", ()).unwrap(),
    _ => {}
  }
}
