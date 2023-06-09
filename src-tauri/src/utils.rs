use tauri::{Manager, Runtime};
use window_shadows::set_shadow;

pub fn set_window_shadow<R:Runtime>(app:&tauri::App<R>){
    let win = app.get_window("mainWindow").unwrap();
    set_shadow(&win,true).expect("Unsupported platform!");
}