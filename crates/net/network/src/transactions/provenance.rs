use alloy_primitives::TxHash;
use reth_network_peers::PeerId;
use std::net::SocketAddr;

/// Pluggable PoG / attribution hook: called after the pool accepts a batch from `peer_id`.
///
/// `listening_addr` is the peer's first-hear advertised listening socket — `(remote_addr.ip(),
/// HelloMessage.port)` captured at session establishment. `None` when the peer signalled
/// `Hello.port == 0` (peer not listening or did not populate the field). See
/// `bera-sentinel/docs/briefs/peer-attribution-enode-pipeline.md` (BERA-305) for end-to-end
/// rationale.
pub trait TransactionProvenanceSink: Send + Sync {
    #[allow(missing_docs)]
    fn record_accepted_from_peer(
        &self,
        peer_id: PeerId,
        listening_addr: Option<SocketAddr>,
        accepted_tx_hashes: &[TxHash],
    );
}
