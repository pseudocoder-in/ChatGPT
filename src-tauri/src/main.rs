
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, RunEvent};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::GlobalShortcutManager;

fn main() {
    let system_tray_menu = SystemTrayMenu::new();
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window: tauri::Window = app.get_window("main").unwrap();
                    // use TrayCenter as initial window position
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                        prepare_chat_gpt_window(window);
                    }
                }
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .setup(|app| {
            //app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            load_chat_gpt(app.handle());
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    app.run(|app_handle, e| match e {
        // Application is ready (triggered only once)
        RunEvent::Ready => {
            let app_handle = app_handle.clone();
            app_handle
                .global_shortcut_manager()
                .register("CmdOrCtrl+Shift+G", move || {
                let app_handle = app_handle.clone();
                let window = app_handle.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                    prepare_chat_gpt_window(window);
                }})
                .unwrap();
        }
        _ => {},
    })

}

fn load_chat_gpt(app_handle: tauri::AppHandle) {
    let main_window = app_handle.get_window("main").unwrap();
    let _ = main_window.eval(&format!("window.location.replace('https://chat.openai.com/chat');"));
}

fn prepare_chat_gpt_window(window: tauri::Window) {
    let _ = window.eval(&format!("document.getElementById('prompt-textarea').select();"));
    let _ = window.eval(&format!("document.querySelectorAll('.sticky').forEach(element => element.style.display = 'none');"));
}