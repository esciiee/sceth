#![no_main]

use risc0_zkvm::guest::env;
use sha2::{Digest, Sha256};
use eth_lc::{types::{primitives::U64, GenericUpdate}, initialize_light_client, types::Bytes32, LightClientBootstrap, LightClientStore};
use serde::{Deserialize, Serialize};

risc0_zkvm::guest::entry!(main);


#[derive(Debug, Deserialize)]
struct TestCase {
    description: String,
    init: LightClientBootstrap,
}

fn main() {
    // let contents = include_str!("../../../test-vectors/ethereum-mainnet-slot-8905792.json");
    // let test_cases = match serde_json::from_str::<Vec<TestCase>>(&contents) {
    //     Ok(data) => data,
    //     Err(_) => panic!("Failed to parse test vectors"),
    // };
    // let init_bootstrap = &test_cases[0].init;
    // let byte: Bytes32 = Bytes32::default();
    // let lc_store = initialize_light_client(byte, init_bootstrap);

    // borsh::to_writer(
    //     &mut env::journal(),
    //     &(
    //         // Note: guest_id shouldn't be needed if only verifying one block. Handling optional
    //         // values in practice would unnecessarily complicate things.
    //         // TODO double check not having guest id be optional is correct.
    //     ),
    // )
    // .unwrap();
}
