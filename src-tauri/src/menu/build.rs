use tauri::{
  image::Image,
  menu::{AboutMetadataBuilder, Menu, MenuBuilder, MenuItem, PredefinedMenuItem, SubmenuBuilder},
  AppHandle, Runtime,
};

pub fn build<R: Runtime>(handle: &AppHandle<R>) -> tauri::Result<Menu<R>> {
  let sm_file = SubmenuBuilder::new(handle, "File")
    .item(&MenuItem::with_id(handle, "new-file", "New File...", true, Some("CmdOrCtrl+N"))?)
    .separator()
    .item(&MenuItem::with_id(handle, "open-file", "Open File...", true, Some("CmdOrCtrl+O"))?)
    .separator()
    .item(&MenuItem::with_id(handle, "save", "Save", true, Some("CmdOrCtrl+S"))?)
    .item(&MenuItem::with_id(handle, "save-as", "Save As...", true, Some("CmdOrCtrl+Shift+S"))?)
    .separator();

  #[cfg(target_os = "linux")]
  let sm_file =
    sm_file.item(&MenuItem::with_id(handle, "close-window", "Close Window", true, Some("Alt+F4"))?);

  #[cfg(not(target_os = "linux"))]
  let sm_file = sm_file.item(&PredefinedMenuItem::quit(handle, None::<&str>)?);

  let sm_edit = SubmenuBuilder::new(handle, "Edit");

  #[cfg(any(target_os = "linux", target_os = "windows"))]
  let sm_edit = sm_edit
    .item(&MenuItem::with_id(handle, "undo", "Undo", true, Some("CmdOrCtrl+Z"))?)
    .item(&MenuItem::with_id(handle, "redo", "Redo", true, Some("CmdOrCtrl+Y"))?)
    .separator();

  #[cfg(not(any(target_os = "linux", target_os = "windows")))]
  let sm_edit = sm_edit
    .item(&PredefinedMenuItem::undo(handle, None::<&str>)?)
    .item(&PredefinedMenuItem::redo(handle, None::<&str>)?)
    .separator();

  let sm_edit = sm_edit
    .item(&PredefinedMenuItem::cut(handle, None::<&str>)?)
    .item(&PredefinedMenuItem::copy(handle, None::<&str>)?)
    .item(&PredefinedMenuItem::paste(handle, None::<&str>)?);

  let sm_view = SubmenuBuilder::new(handle, "View")
    .item(&PredefinedMenuItem::fullscreen(handle, None::<&str>)?)
    .separator()
    .item(&MenuItem::with_id(handle, "zoom-in", "Zoom In", true, Some("CmdOrCtrl+Shift+="))?)
    .item(&MenuItem::with_id(handle, "zoom-out", "Zoom Out", true, Some("CmdOrCtrl+Shift+-"))?)
    .item(&MenuItem::with_id(handle, "reset-zoom", "Reset Zoom", true, Some("CmdOrCtrl+Shift+0"))?);

  let menu = MenuBuilder::new(handle)
    .item(&sm_file.build()?)
    .item(&sm_edit.build()?)
    .item(&sm_view.build()?)
    .item(
      &SubmenuBuilder::new(handle, "Help")
        .about(Some(
          AboutMetadataBuilder::new()
            .icon(Some(Image::from_bytes(include_bytes!("../../icons/StoreLogo.png")).unwrap()))
            .name(Some("NTR Pad"))
            .authors(Some(vec!["ahas <dev@ahas.io>".to_string()]))
            .website(Some("https://github.com/ahas/ntrpad"))
            .comments(Some("Nuxt 3 + Tauri 2.0 Notepad Example"))
            .license(Some("MIT"))
            .build(),
        ))
        .build()?,
    )
    .build()?;

  Ok(menu)
}
