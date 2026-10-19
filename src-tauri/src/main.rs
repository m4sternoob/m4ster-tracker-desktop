// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Emitter, Manager};
use tauri_plugin_deep_link::DeepLinkExt;
use url::Url;

/// Handle `m4ster-tracker://` deep links.
/// Used for Google OAuth callback: after sign-in in the system browser,
/// the backend redirects to `m4ster-tracker://auth/callback?code=...&state=...`
/// which the frontend listens for via the `deep-link-url` event.
fn handle_deep_link(app: &tauri::AppHandle, urls: Vec<Url>) {
    for url in urls {
        println!("[deep-link] {}", url.as_str());
        let _ = app.emit("deep-link-url", url.as_str());
    }
    // Focus the existing window when a deep link arrives.
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_focus();
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_updater::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Second instance launched (e.g. via deep link): focus existing window.
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let handle = app.handle().clone();
            app.deep_link().on_open_url(move |event| {
                handle_deep_link(&handle, event.urls());
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
