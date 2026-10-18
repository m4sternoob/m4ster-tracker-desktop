// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_deep_link::DeepLinkExt;
use url::Url;

/// Handle `m4ster-tracker://` deep links.
/// Used for Google OAuth callback: after sign-in in the system browser,
/// Google redirects to `m4ster-tracker://auth/callback?code=...&state=...`
/// which the frontend listens for via the `deep-link` event.
fn handle_deep_link(urls: Vec<Url>) {
    for url in urls {
        // Forward to frontend via event; the React app completes the OAuth flow.
        // The actual token exchange happens against the Vercel backend.
        println!("[deep-link] {}", url.as_str());
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .setup(|app| {
            // Register deep-link listener: forward URLs to the frontend.
            let handle = app.handle().clone();
            app.deep_link().on_open_url(move |event| {
                let urls = event.urls();
                handle_deep_link(urls.clone());
                // Emit to frontend so React can finish OAuth.
                for url in urls {
                    let _ = handle.emit("deep-link-url", url.as_str());
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
