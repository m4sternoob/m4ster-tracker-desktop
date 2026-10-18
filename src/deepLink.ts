/**
 * Desktop deep-link helper for M4ster Tracker (Tauri).
 *
 * Google OAuth flow on desktop:
 * 1. Frontend calls `startGoogleSignIn()` → opens system browser to the
 *    Vercel backend's Google OAuth endpoint.
 * 2. User signs in with Google in the browser.
 * 3. Backend redirects to `m4ster-tracker://auth/callback?code=...&state=...`
 * 4. OS routes the deep link to this app; Rust emits `deep-link-url`.
 * 5. This module listens for that event and completes sign-in by
 *    exchanging the code via the Vercel backend.
 *
 * The web app (https://m4ster-tracker.vercel.app) uses the standard
 * redirect flow; this module is only bundled/used in the Tauri desktop
 * build. On web, `isTauri()` returns false and this is a no-op.
 */

const DEEP_LINK_EVENT = 'deep-link-url';
const BACKEND = 'https://m4ster-tracker.vercel.app';

export function isTauri(): boolean {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

type DeepLinkHandler = (url: string) => void;
const handlers = new Set<DeepLinkHandler>();

/** Subscribe to deep-link URLs (Tauri desktop only). Returns unsubscribe. */
export async function onDeepLink(handler: DeepLinkHandler): Promise<() => void> {
  if (!isTauri()) return () => {};
  const { listen } = await import('@tauri-apps/api/event');
  const unlisten = await listen<string>(DEEP_LINK_EVENT, (event) => {
    handler(event.payload);
  });
  handlers.add(handler);
  return () => {
    handlers.delete(handler);
    unlisten();
  };
}

/** Open the system browser for Google sign-in (Tauri desktop only). */
export async function startGoogleSignIn(): Promise<void> {
  if (!isTauri()) {
    // Fall back to web flow.
    window.location.href = `${BACKEND}/api/auth/google`;
    return;
  }
  const { openUrl } = await import('@tauri-apps/plugin-opener');
  // `desktop=1` tells the backend to redirect to the deep-link scheme.
  await openUrl(`${BACKEND}/api/auth/google?desktop=1`);
}

/**
 * Parse an OAuth callback deep link.
 * Returns { code, state } or null if the URL isn't an auth callback.
 */
export function parseAuthCallback(url: string): { code: string; state: string } | null {
  try {
    const u = new URL(url);
    if (u.protocol !== 'm4ster-tracker:') return null;
    // m4ster-tracker://auth/callback?code=...&state=...
    if (!u.hostname.startsWith('auth')) return null;
    const code = u.searchParams.get('code');
    const state = u.searchParams.get('state');
    if (!code) return null;
    return { code, state: state ?? '' };
  } catch {
    return null;
  }
}
