
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, SystemTray, SystemTrayEvent, RunEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_positioner::{Position, WindowExt};
use tauri::GlobalShortcutManager;
use std::sync::Mutex;

#[derive(Debug)]
#[derive(Clone)]
enum AssistantType {
    ChatGPT,
    Gemini,
    Copilot,
}

static ASSIST_ENUM: Mutex<AssistantType> = Mutex::new(AssistantType::ChatGPT);


fn main() {
    let chatgpt = CustomMenuItem::new("chatgpt".to_string(), "ChatGPT").selected();
    let gemini = CustomMenuItem::new("gemini".to_string(), "Gemini");
    let copilot = CustomMenuItem::new("copilot".to_string(), "Copilot");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let system_tray_menu = SystemTrayMenu::new()
    .add_item(chatgpt)
    .add_item(gemini)
    .add_item(copilot)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);
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
                        //load_chat_gpt(app.clone());
                        prepare_chat_gpt_window(window);
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    app.tray_handle().get_item("chatgpt").set_selected(false).unwrap();
                    app.tray_handle().get_item("gemini").set_selected(false).unwrap();
                    app.tray_handle().get_item("copilot").set_selected(false).unwrap();
                    match id.as_str() {
                      "quit" => {
                        std::process::exit(0);
                      }
                      "chatgpt" => {
                        {
                            let mut global_enum = ASSIST_ENUM.lock().unwrap();
                            *global_enum = AssistantType::ChatGPT;
                        }
                        load_chat_gpt(app.clone());
                        let item_handle = app.tray_handle().get_item(&id);
                        item_handle.set_selected(true).unwrap();
                      }
                      "gemini" => {
                        {
                            let mut global_enum = ASSIST_ENUM.lock().unwrap();
                            *global_enum = AssistantType::Gemini;
                        }
                        load_chat_gpt(app.clone());
                        let item_handle: tauri::SystemTrayMenuItemHandle = app.tray_handle().get_item(&id);
                        item_handle.set_selected(true).unwrap();
                      }
                      "copilot" => {
                        {
                            let mut global_enum = ASSIST_ENUM.lock().unwrap();
                            *global_enum = AssistantType::Copilot;
                        }
                        load_chat_gpt(app.clone());
                        let item_handle: tauri::SystemTrayMenuItemHandle = app.tray_handle().get_item(&id);
                        item_handle.set_selected(true).unwrap();
                      }
                      _ => {}
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
            let main_window = app.get_window("main").unwrap();
            //app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
                use cocoa::base::id;
                let ns_win = main_window.ns_window().unwrap() as id;
                unsafe {
                    ns_win.setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
                }
            }
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
    let current_value = ASSIST_ENUM.lock().unwrap();
    match *current_value {
        AssistantType::ChatGPT => {
            let _ = main_window.eval(&format!("window.location.replace('https://chat.openai.com/chat');"));
        }
        AssistantType::Gemini => {
            let _ = main_window.eval(&format!("window.location.replace('https://gemini.google.com/app');"));
        }
        AssistantType::Copilot => {
            let _ = main_window.eval(&format!("window.location.replace('https://copilot.microsoft.com/');"));
        }
    }
}

fn prepare_chat_gpt_window(window: tauri::Window) {
    let current_value = ASSIST_ENUM.lock().unwrap();
    match *current_value {
        AssistantType::ChatGPT => {
            let _ = window.eval(&format!("document.getElementById('prompt-textarea').select();"));
            let _ = window.eval(&format!("document.querySelectorAll('.sticky').forEach(element => element.style.display = 'none');"));
        }
        AssistantType::Gemini => {
            //let _ = window.eval(&format!("document.querySelectorAll('.boqOnegoogleliteOgbOneGoogleBar').forEach(element => element.style.display = 'none');"));
        }
        AssistantType::Copilot => {
        }
    }
}