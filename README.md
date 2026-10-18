# M4ster Tracker — Desktop

Native **Windows** (.exe) and **macOS** (.dmg) app for M4ster Tracker, built with [Tauri](https://tauri.app/) (Rust backend + React frontend).

Same Vercel backend as the web app — your boards, Google sign-in, and data sync across web and desktop automatically.

- **Web:** https://m4ster-tracker.vercel.app
- **Backend:** Vercel serverless + Upstash Redis (shared)
- **Desktop:** this repo (Tauri v2)

## What it is

A dedicated desktop GUI for M4ster Tracker — like Figma's desktop app. The React UI is bundled locally for instant load; all data flows through the existing Vercel API, so web and desktop stay in sync.

**v1 scope:**
- Native window (1400×900, min 1000×700), app icon, system menus
- Windows NSIS installer (`.exe` setup), macOS DMG (`.dmg`)
- Deep-link protocol `m4ster-tracker://` for Google OAuth callback
- Google sign-in via system browser → deep link → token stored, synced via backend

## Architecture

```
m4ster-tracker-desktop/
├── src-tauri/           # Rust backend
│   ├── main.rs          # Entry point, deep-link handler
│   ├── tauri.conf.json  # App config (windows, bundler, deep links)
│   ├── Cargo.toml       # Rust dependencies
│   └── icons/           # App icons (MT neon logo)
├── src/
│   └── deepLink.ts      # Frontend deep-link + OAuth helper
├── scripts/
│   └── sync-frontend.sh # Clones + builds web UI into dist/
├── dist/                # Built React app (generated, gitignored)
└── frontend-src/        # Cloned web repo (generated, gitignored)
```

**Data flow:** Desktop UI (local) → Vercel API (`https://m4ster-tracker.vercel.app/api/*`) → Upstash Redis. Same as web. No separate backend.

**Auth flow (desktop):**
1. User clicks "Sign in with Google" → `startGoogleSignIn()` opens system browser
2. Google OAuth happens in browser → backend redirects to `m4ster-tracker://auth/callback?code=...`
3. OS routes deep link to app → Rust emits `deep-link-url` event
4. Frontend (`deepLink.ts`) captures code → exchanges via backend → session synced

> **Note:** The backend needs a `/api/auth/google?desktop=1` endpoint that redirects to the deep-link scheme instead of the web callback URL. Until that's added, desktop Google sign-in falls back to opening the web app sign-in (manual token copy). See `src/deepLink.ts`.

## Prerequisites

- **Rust** (stable): https://rustup.rs
- **Node.js** 20+
- **Platform deps:**
  - Windows: WebView2 (preinstalled on Win10/11), NSIS for installer
  - macOS: Xcode Command Line Tools

## Build

```bash
# Install
npm install

# Sync + build the React frontend from the web repo
npm run sync-frontend

# Dev (hot reload)
npm run dev

# Build installers
npx tauri build
```

**Outputs:**
- Windows: `src-tauri/target/release/bundle/nsis/M4ster_Tracker_0.1.0_x64-setup.exe`
- macOS: `src-tauri/target/release/bundle/dmg/M4ster_Tracker_0.1.0_x64.dmg` (Intel) / `_aarch64.dmg` (Apple Silicon)

> Cross-compiling: build Windows installers on Windows, macOS DMGs on macOS. Use GitHub Actions for both (see `.github/workflows/`).

## Configuration

`src-tauri/tauri.conf.json` highlights:

| Key | Value |
|---|---|
| App name | M4ster Tracker |
| Identifier | `com.m4ster.tracker` |
| Window | 1400×900, min 1000×700 |
| Deep link | `m4ster-tracker://` |
| Windows bundler | NSIS (per-machine) |
| macOS bundler | DMG |

## Status

- [x] Tauri v2 scaffold (Rust + config + icons)
- [x] Deep-link protocol registered (`m4ster-tracker://`)
- [x] Deep-link Rust handler + frontend helper
- [x] Frontend sync script (builds web UI locally)
- [x] NSIS + DMG bundler config
- [ ] Backend `/api/auth/google?desktop=1` endpoint (needs web repo change)
- [ ] End-to-end Google OAuth test on Windows + macOS
- [ ] GitHub Actions CI (build .exe + .dmg on release)
- [ ] Auto-updater

## Related

- Web app repo: [m4sternoob/m4ster-roadmap-tracker](https://github.com/m4sternoob/m4ster-roadmap-tracker)
- Live web: https://m4ster-tracker.vercel.app

## License

MIT
