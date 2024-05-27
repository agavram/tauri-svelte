#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(non_upper_case_globals)]
const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
#[allow(non_upper_case_globals)]
const NSWindowStyleMaskResizable: i32 = 1 << 3;

use cocoa::appkit::NSWindowCollectionBehavior;
use objc::{msg_send, sel, sel_impl};
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    Manager, WebviewWindow,
};
use tauri_nspanel::{panel_delegate, ManagerExt, WebviewWindowExt};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

mod spotlight;

fn main() {
    // let quit = tauri::tray:: CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    // let tray_menu = SystemTrayMenu::new().add_item(quit);
    // let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state == ShortcutState::Released {
                        return;
                    }
                    let panel = app.app_handle().get_webview_panel("main").unwrap();
                    if panel.is_visible() {
                        panel.order_out(None);
                    } else {
                        panel.show();
                    }
                })
                .build(),
        )
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![spotlight::init_spotlight_window,])
        .manage(spotlight::State::default())
        .setup(move |app| {
            let app_handle = app.app_handle();
            let window: WebviewWindow = app_handle.get_webview_window("main").unwrap();
            let _ = window.set_min_size(Some(tauri::LogicalSize {
                width: 400,
                height: 450,
            }));
            let panel = window.to_panel().unwrap();

            panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel | NSWindowStyleMaskResizable);
            panel.set_collection_behaviour(
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
            );
            app.global_shortcut().register("Alt+Space").unwrap();

            let delegate = panel_delegate!(MyPanelDelegate {
                window_did_resign_key
            });

            let p = panel.clone();
            delegate.set_listener(Box::new(move |delegate_name: String| {
                p.order_out(None);
            }));
            // Set your panel's delegate
            panel.set_delegate(delegate);

            let quit = MenuItemBuilder::with_id("quit", "Quit".to_string())
                .accelerator("Cmd+Q")
                .build(app)?;
            let menu = MenuBuilder::new(app).items(&[&quit]).build()?;
            let tray = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        app.app_handle().exit(0);
                    }
                    _ => (),
                })
                .icon(Image::from_path("icons/icon.png").unwrap())
                .build(app)?;
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                // api.prevent_exit();
            }
            _ => {}
        });
}
