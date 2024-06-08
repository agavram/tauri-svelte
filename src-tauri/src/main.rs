#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(non_upper_case_globals)]
const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
#[allow(non_upper_case_globals)]
const NSWindowStyleMaskResizable: i32 = 1 << 3;

use std::ffi::CString;

use cocoa::{
    appkit::{NSMainMenuWindowLevel, NSWindowCollectionBehavior},
    base::{id, nil},
};
use objc::{class, msg_send, sel, sel_impl};
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, WebviewWindow, Wry,
};
use tauri_nspanel::{block::ConcreteBlock, panel_delegate, ManagerExt, WebviewWindowExt};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_shell::ShellExt;

mod spotlight;

fn get_frontmost_app_pid() -> i32 {
    let workspace: id = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };

    let frontmost_application: id = unsafe { msg_send![workspace, frontmostApplication] };

    let pid: i32 = unsafe { msg_send![frontmost_application, processIdentifier] };

    pid
}

fn app_pid() -> i32 {
    let process_info: id = unsafe { msg_send![class!(NSProcessInfo), processInfo] };

    let pid: i32 = unsafe { msg_send![process_info, processIdentifier] };

    pid
}

fn register_workspace_listener(name: String, callback: Box<dyn Fn()>) {
    let workspace: id = unsafe { msg_send![class!(NSWorkspace), sharedWorkspace] };

    let notification_center: id = unsafe { msg_send![workspace, notificationCenter] };

    let block = ConcreteBlock::new(move |_notif: id| {
        callback();
    });

    let block = block.copy();

    let name: id =
        unsafe { msg_send![class!(NSString), stringWithCString: CString::new(name).unwrap()] };

    unsafe {
        let _: () = msg_send![
            notification_center,
            addObserverForName: name object: nil queue: nil usingBlock: block
        ];
    }
}

fn hide_menubar_panel(app_handle: &tauri::AppHandle) {
    let panel = app_handle.get_webview_panel("main").unwrap();
    // if get_frontmost_app_pid() == app_pid() {
    //     panel.order_out(None);
    //     panel.show();
    //     return;
    // }

    panel.order_out(None);
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app: &AppHandle, shortcut, event| {
                    if event.state == ShortcutState::Released {
                        return;
                    }
                    let panel = app.app_handle().get_webview_panel("main").unwrap();
                    let window = app.app_handle().get_webview_window("main").unwrap();
                    if panel.is_visible() {
                        panel.order_out(None);
                    } else {
                        panel.show();
                    }
                })
                .build(),
        )
        .plugin(tauri_nspanel::init())
        .invoke_handler(tauri::generate_handler![hide_spotlight])
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
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
            );
            panel.set_level(NSMainMenuWindowLevel + 1);
            app.global_shortcut().register("Alt+Space").unwrap();

            let handle = app_handle.clone();
            let handle2 = app_handle.clone();

            app_handle.listen_any("menubar_panel_did_resign_key", move |_| {
                hide_menubar_panel(&handle2);
            });

            let callback = Box::new(move || {
                hide_menubar_panel(&handle);
            });

            register_workspace_listener(
                "NSWorkspaceDidActivateApplicationNotification".into(),
                callback.clone(),
            );

            register_workspace_listener(
                "NSWorkspaceActiveSpaceDidChangeNotification".into(),
                callback,
            );

            let delegate = panel_delegate!(MyPanelDelegate {
                window_did_resign_key
            });

            let handle = app_handle.clone();
            delegate.set_listener(Box::new(move |delegate_name: String| {
                match delegate_name.as_str() {
                    "window_did_resign_key" => {
                        let _ = handle.emit("menubar_panel_did_resign_key", ());
                    }
                    _ => (),
                }
            }));
            // Set your panel's delegate
            panel.set_delegate(delegate);

            let quit = MenuItemBuilder::with_id("quit", "Quit".to_string())
                .accelerator("Cmd+Q")
                .build(app)?;
            let menu = MenuBuilder::new(app).items(&[&quit]).build()?;
            let _ = TrayIconBuilder::new()
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "quit" => {
                        app.app_handle().exit(0);
                    }
                    _ => (),
                })
                .icon(Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap())
                .build(app)?;
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { .. } => {
                // api.prevent_exit();
            }
            _ => {}
        });
}

#[tauri::command]
fn hide_spotlight(window: tauri::Window) {
    window.get_webview_panel("main").unwrap().order_out(None);
}
