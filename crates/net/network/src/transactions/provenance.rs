use alloy_primitives::TxHash;
use reth_network_peers::PeerId;

/// Pluggable PoG / attribution hook: called after the pool accepts a batch from `peer_id`.
pub trait TransactionProvenanceSink: Send + Sync {
    #[allow(missing_docs)]
    fn record_accepted_from_peer(&self, peer_id: PeerId, accepted_tx_hashes: &[TxHash]);
}
