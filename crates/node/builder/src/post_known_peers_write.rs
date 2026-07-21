//! Optional **post-**`known_peers_write` hook: runs after Reth persists `known-peers.json` on
//! graceful network shutdown.

use std::{
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

type Hook = Arc<dyn Fn(&Path) + Send + Sync + 'static>;
type HookRegistration = Box<dyn Fn(&Path) + Send + Sync + 'static>;

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
pub fn set_post_known_peers_write_hook(hook: Option<HookRegistration>) {
    let mut guard = hook_slot().lock().unwrap_or_else(|e| e.into_inner());
    *guard = hook.map(Arc::from);
}

pub(crate) fn run_post_known_peers_write_hook(path: &Path) {
    let hook = {
        let guard = hook_slot().lock().unwrap_or_else(|e| e.into_inner());
        guard.clone()
    };
    if let Some(hook) = hook {
        hook(path);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::Path,
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
    };

    use super::{run_post_known_peers_write_hook, set_post_known_peers_write_hook};

    #[test]
    fn hook_can_replace_registration_without_deadlocking() {
        let called = Arc::new(AtomicBool::new(false));
        let called_for_hook = Arc::clone(&called);
        set_post_known_peers_write_hook(Some(Box::new(move |_| {
            called_for_hook.store(true, Ordering::Relaxed);
            set_post_known_peers_write_hook(None);
        })));

        run_post_known_peers_write_hook(Path::new("known-peers.json"));

        assert!(called.load(Ordering::Relaxed));
    }
}
