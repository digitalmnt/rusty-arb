//! Collectors are responsible for collecting data from external sources and
//! turning them into internal events. For example, a collector might listen to
//! a stream of new blocks, and turn them into a stream of `NewBlock` events.

/// This collector listens to a stream of new blocks.
pub mod block_collector;

/// This collector listens to a stream of new pending transactions.
pub mod mempool_collector;

// mev share collector that listens for new mev share events
pub mod mevshare_collector;
