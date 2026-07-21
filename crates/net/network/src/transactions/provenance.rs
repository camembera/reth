use alloy_primitives::TxHash;
use reth_network_peers::PeerId;
use std::net::SocketAddr;

/// Pluggable PoG / attribution hook: called after the pool accepts a batch from `peer_id`.
pub trait TransactionProvenanceSink: Send + Sync {
    /// Records transactions accepted from a peer.
    ///
    /// `listening_addr` uses port zero when the peer's remote IP is known but it did not advertise
    /// a listening port. Such an address is valid for attribution but must not be treated as
    /// redialable.
    fn record_accepted_from_peer(
        &self,
        peer_id: PeerId,
        listening_addr: Option<SocketAddr>,
        accepted_tx_hashes: &[TxHash],
    );
}
