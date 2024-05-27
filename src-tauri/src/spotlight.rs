use std::sync::{Mutex, Once};

use bitflags::bitflags;

use objc_id::{Id, ShareId};
use tauri::{
    AppHandle, Manager, PhysicalPosition, PhysicalSize, Window, Wry,
};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
use tauri_nspanel::WebviewWindowExt;


use cocoa::{
    appkit::{
        CGFloat, NSColor, NSView, NSViewHeightSizable, NSViewWidthSizable,
        NSWindowCollectionBehavior,
    },
    base::{id, nil, BOOL, NO, YES},
    foundation::{NSPoint, NSRect, NSSize, NSString},
};
use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{self, Class, Object, Protocol, Sel},
    sel, sel_impl, Message,
};
use objc_foundation::INSObject;

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    pub fn NSMouseInRect(aPoint: NSPoint, aRect: NSRect, flipped: BOOL) -> BOOL;
}

bitflags! {
    struct NSTrackingAreaOptions: u32 {
        const NSTrackingActiveAlways = 0x80;
        const NSTrackingMouseEnteredAndExited = 0x01;
        const NSTrackingMouseMoved = 0x02;
        const NSTrackingCursorUpdate = 0x04;
    }
}

#[derive(Default)]
pub struct Store {
    // panel: Option<ShareId<RawNSPanel>>,
}

#[derive(Default)]
pub struct State(pub Mutex<Store>);

#[macro_export]
macro_rules! set_state {
    ($app_handle:expr, $field:ident, $value:expr) => {{
        let handle = $app_handle.app_handle();
        handle
            .state::<$crate::spotlight::State>()
            .0
            .lock()
            .unwrap()
            .$field = $value;
    }};
}

#[macro_export]
macro_rules! get_state {
    ($app_handle:expr, $field:ident) => {{
        let handle = $app_handle.app_handle();
        let value = handle
            .state::<$crate::spotlight::State>()
            .0
            .lock()
            .unwrap()
            .$field;

        value
    }};
    ($app_handle:expr, $field:ident, $action:ident) => {{
        let handle = $app_handle.app_handle();
        let value = handle
            .state::<$crate::spotlight::State>()
            .0
            .lock()
            .unwrap()
            .$field
            .$action();

        value
    }};
}

static INIT: Once = Once::new();
static PANEL_LABEL: &str = "main";

#[tauri::command]
pub fn init_spotlight_window(app_handle: AppHandle<Wry>, window: Window<Wry>) {
    let w = window.get_webview_window("main").unwrap();
    let p = w.to_panel().unwrap();
    // INIT.call_once(|| {
    //     set_state!(app_handle, panel, Some(create_spotlight_panel(&window)));
    //     register_shortcut(app_handle);
    // });
}

// fn register_shortcut(app_handle: AppHandle<Wry>) {
//     let mut shortcut_manager = app_handle.global_shortcut_manager();
//     let window = app_handle.get_window(PANEL_LABEL).unwrap();

//     let panel = panel!(app_handle);
//     shortcut_manager
//         .register("Cmd+k", move || {
//             position_window_at_the_center_of_the_monitor_with_cursor(&window);

//             if panel.is_visible() {
//                 hide_spotlight(window.app_handle());
//             } else {
//                 show_spotlight(window.app_handle());
//             };
//         })
//         .unwrap();
// }

// #[tauri::command]
// pub fn show_spotlight(app_handle: AppHandle<Wry>) {
//     panel!(app_handle).show();
// }

// #[tauri::command]
// pub fn hide_spotlight(app_handle: AppHandle<Wry>) {
//     panel!(app_handle).order_out(None);
// }

/// Positions a given window at the center of the monitor with cursor
fn position_window_at_the_center_of_the_monitor_with_cursor(window: &Window<Wry>) {
    if let Some(monitor) = get_monitor_with_cursor() {
        let handle: id = window.ns_window().unwrap() as _;
        let win_frame: NSRect = unsafe { NSView::frame(handle) };
        let size = NSSize::new(win_frame.size.width, win_frame.size.height);

        let origin = NSPoint {
            x: win_frame.origin.x,
            y: win_frame.origin.y,
        };

        let mut rect = NSRect {
            origin: origin,
            size: size,
        };

        if !is_origin_visible(origin) {
            println!("Not visible!");
            let display_size = monitor.size.to_logical::<f64>(monitor.scale_factor);
            let display_pos = monitor.position.to_logical::<f64>(monitor.scale_factor);
            let size = NSSize::new(600.0, 700.0);

            rect = NSRect {
                origin: NSPoint {
                    x: (display_pos.x + (display_size.width / 2.0)) - (size.width / 2.0),
                    y: (display_pos.y + (display_size.height / 2.0)) - (size.height / 2.0),
                },
                size: size,
            };
        }

        let _: () = unsafe { msg_send![handle, setFrame: rect display: YES] };
    }
}

fn is_origin_visible(origin: NSPoint) -> bool {
    objc::rc::autoreleasepool(|| {
        let screens: id = unsafe { msg_send![class!(NSScreen), screens] };
        let screens_iter: id = unsafe { msg_send![screens, objectEnumerator] };
        let mut next_screen: id;

        loop {
            next_screen = unsafe { msg_send![screens_iter, nextObject] };
            if next_screen == nil {
                break;
            }

            let frame: NSRect = unsafe { msg_send![next_screen, frame] };
            let is_mouse_in_screen_frame: BOOL = unsafe { NSMouseInRect(origin, frame, NO) };
            if is_mouse_in_screen_frame == YES {
                return true;
            }
        }
        return false;
    })
}

struct Monitor {
    #[allow(dead_code)]
    pub name: Option<String>,
    pub size: PhysicalSize<u32>,
    pub position: PhysicalPosition<i32>,
    pub scale_factor: f64,
}

/// Gets the Monitor with cursor
fn get_monitor_with_cursor() -> Option<Monitor> {
    objc::rc::autoreleasepool(|| {
        let mouse_location: NSPoint = unsafe { msg_send![class!(NSEvent), mouseLocation] };
        let screens: id = unsafe { msg_send![class!(NSScreen), screens] };
        let screens_iter: id = unsafe { msg_send![screens, objectEnumerator] };
        let mut next_screen: id;

        let frame_with_cursor: Option<NSRect> = loop {
            next_screen = unsafe { msg_send![screens_iter, nextObject] };
            if next_screen == nil {
                break None;
            }

            let frame: NSRect = unsafe { msg_send![next_screen, frame] };
            let is_mouse_in_screen_frame: BOOL =
                unsafe { NSMouseInRect(mouse_location, frame, NO) };
            if is_mouse_in_screen_frame == YES {
                break Some(frame);
            }
        };

        if let Some(frame) = frame_with_cursor {
            let _: id = unsafe { msg_send![next_screen, localizedName] };
            let device_description: id = unsafe { msg_send![next_screen, deviceDescription] };
            let screen_number: id = unsafe {
                msg_send![device_description, objectForKey: NSString::alloc(nil).init_str("NSScreenNumber")]
            };
            let screen_number: i32 = unsafe { msg_send![screen_number, intValue] };
            let scale_factor: CGFloat = unsafe { msg_send![next_screen, backingScaleFactor] };
            let scale_factor: f64 = scale_factor;

            return Some(Monitor {
                name: Some(screen_number.to_string()),
                position: PhysicalPosition {
                    x: (frame.origin.x * scale_factor) as i32,
                    y: (frame.origin.y * scale_factor) as i32,
                },
                size: PhysicalSize {
                    width: (frame.size.width * scale_factor) as u32,
                    height: (frame.size.height * scale_factor) as u32,
                },
                scale_factor,
            });
        }

        None
    })
}

extern "C" {
    pub fn object_setClass(obj: id, cls: id) -> id;
}

#[allow(non_upper_case_globals)]
const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

#[allow(non_upper_case_globals)]
const NSWindowStyleMaskResizable: i32 = 1 << 3;

const CLS_NAME: &str = "RawNSPanel";

fn create_spotlight_panel(window: &Window<Wry>) {
    // Convert NSWindow Object to NSPanel
    let handle: id = window.ns_window().unwrap() as _;
    let w = window.ns_window().unwrap();
    // let panel = window
    // let panel = RawNSPanel::from(handle);
    // let panel = panel.share();

    // #[cfg(target_os = "macos")]
    // apply_vibrancy(
    //     &w,
    //     NSVisualEffectMaterial::ContentBackground,
    //     None,
    //     Some(10.0),
    // )
    // .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    // panel.set_auto_hide(true);

    // // Ensure that the panel can display over the top of fullscreen apps
    // panel.set_collection_behaviour(
    //     NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace
    //         | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
    // );

    // // Ensures panel does not activate
    // // panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);
    // panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel | NSWindowStyleMaskResizable);

    // let min_size = NSSize::new(400.0, 450.0);
    // let () = unsafe { msg_send![panel, setMinSize: min_size] };

    // // Setup delegate for an NSPanel to listen for window resign key and hide the panel
    // let delegate = RawNSPanelDelegate::new();
    // delegate.set_panel_(panel.clone());
    // panel.set_delegate(Some(delegate));

    // // On older macOS i.e on (12.3), hover detection is not working, see https://github.com/ahkohd/tauri-macos-spotlight-example/issues/14
    // // To fix this, add a tracking view to the panel
    // let view: id = panel.content_view();

    // unsafe {
    //     // let _: () = view.setHasShadow_(NO);
    //     let clear_color: id = NSColor::clearColor(nil);
    //     let _: () = msg_send![panel, setBackgroundColor: clear_color];
    //     let _: () = msg_send![panel, setOpaque: NO];
    //     let () = msg_send![view, setWantsLayer: YES];
    //     let _: id = msg_send![view, layer];
    // }

    // let bound: NSRect = unsafe { NSView::bounds(view) };
    // let track_view: id = unsafe { msg_send![class!(NSTrackingArea), alloc] };
    // let track_view: id = unsafe {
    //     msg_send![
    //         track_view,
    //         initWithRect: bound
    //         options: NSTrackingAreaOptions::NSTrackingActiveAlways
    //         | NSTrackingAreaOptions::NSTrackingMouseEnteredAndExited
    //         | NSTrackingAreaOptions::NSTrackingMouseMoved
    //         | NSTrackingAreaOptions::NSTrackingCursorUpdate
    //         owner: view
    //         userInfo: nil
    //     ]
    // };
    // let auto_resizing_mask = NSViewWidthSizable | NSViewHeightSizable;
    // let () = unsafe { msg_send![view, setAutoresizingMask: auto_resizing_mask] };
    // let () = unsafe { msg_send![view, addTrackingArea: track_view] };
}
