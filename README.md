# M4ster Tracker — Native Apps

Native Windows, macOS and Android apps for M4ster Tracker — real programs, not web wrappers. Each platform is built in its own native stack against the same tracker API, so boards, sign-in and multiplayer stay in sync with the web app.

- **Web app:** https://m4ster-tracker.vercel.app
- **Web repo:** [m4sternoob/m4ster-roadmap-tracker](https://github.com/m4sternoob/m4ster-roadmap-tracker)
- **Backend:** the web app's Vercel serverless API + Upstash Redis — shared, no separate backend

## Direction

| Platform | Stack | Order |
|---|---|---|
| Windows (.exe) | C++ (Win32) | 1st |
| macOS (.dmg) | Swift (SwiftUI) | 2nd |
| Android (.apk) | Kotlin (Jetpack Compose) | 3rd |

iOS follows later, same native approach. Inside each app the build order is: board core (columns, cards, drag) → the remaining views → multiplayer (presence, chat) last.

## Status

**In development — no releases yet.** The website's Download section picks up releases from this repo automatically: it reads the latest GitHub release and enables each platform's button when an asset with the matching extension (`.exe` / `.dmg` / `.apk`) is attached. Until then the buttons honestly read "In development".

## What came before

This repo previously held a Tauri v2 prototype (Rust shell rendering the web UI). That direction was dropped in October 2026: a wrapper that renders the website is not a native app. The Tauri code remains in git history for reference only — don't build on it.

## Auth

Google sign-in runs in the system browser; the session is exchanged through the web API's desktop endpoints (`api/auth/desktop/` in the web repo) and stored natively on each platform.

## License

MIT
