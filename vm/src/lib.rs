#![allow(clippy::type_complexity)]
#![allow(clippy::derivable_impls)] // TODO: they were fixed in one of the branches, delete when everything gets merged
#![feature(exhaustive_patterns)]

pub mod api;
pub mod bech32;
pub mod display_util;
pub mod mem_conv;
pub mod tx_execution;
pub mod tx_mock;
pub mod vm_hooks;
pub mod world_mock;

pub use tx_mock::DebugApi;
pub use world_mock::BlockchainMock;

// Re-exporting for convenience. Using the crate as imported in the codec to make sure the save version is used everywhere.
pub use multiversx_sc::codec::num_bigint;

// Re-exporting the executor, for convenience.
pub use multiversx_chain_vm_executor as executor;

#[macro_use]
extern crate alloc;
