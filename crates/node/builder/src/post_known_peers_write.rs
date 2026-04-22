//! Optional **post-**`known_peers_write` hook: runs after Reth persists `known-peers.json` on graceful
//! network shutdown.

use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

type Hook = Box<dyn Fn(&Path) + Send + Sync + 'static>;

static HOOK_SLOT: OnceLock<Mutex<Option<Hook>>> = OnceLock::new();

fn hook_slot() -> &'static Mutex<Option<Hook>> {
    HOOK_SLOT.get_or_init(|| Mutex::new(None))
}

/// Register a hook to run immediately **after** a successful `known-peers.json` write during
/// graceful network shutdown (right after [`reth_network::NetworkManager::write_peers_to_file`]).
/// Pass `None` to clear any previously registered hook.
///
/// Lets downstream binaries post-process the peer list without racing the built-in persistence
/// pass (which runs from the CLI graceful-shutdown path after the node command future returns).
pub fn set_post_known_peers_write_hook(hook: Option<Hook>) {
    let mut guard = hook_slot().lock().unwrap_or_else(|e| e.into_inner());
    *guard = hook;
}

pub(crate) fn run_post_known_peers_write_hook(path: &Path) {
    let guard = hook_slot().lock().unwrap_or_else(|e| e.into_inner());
    if let Some(hook) = guard.as_ref() {
        hook(path);
    }
}
