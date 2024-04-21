use std::fs::File;

use eth_lc::{
    initialize_light_client,
    types::{primitives::U64, Bytes32, GenericUpdate, Update},
    LightClientBootstrap, LightClientStore,
};
use methods::{LIGHT_CLIENT_ELF, LIGHT_CLIENT_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ExpectedParams {
    is_valid: bool,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Params {
//     previous_block: LightClientBlockLiteView,
//     current_bps: Vec<ValidatorStakeView>,
//     new_block: LightClientBlockView,
// }

#[derive(Debug, Deserialize)]
struct TestCase {
    description: String,
    init: LightClientBootstrap,
    updates: Vec<GenericUpdate>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Params {
    lc_store: LightClientStore,
    update: GenericUpdate,
    slot: U64,
    genesis_validotors: Bytes32,
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
    let contents = include_str!("../../test-vectors/ethereum-mainnet-slot-8905792.json");
    let test_cases: Vec<TestCase> = serde_json::from_str(&contents)?;
    let init_bootstrap = &test_cases[0].init;
    let byte: Bytes32 = Bytes32::default();
    let lc_store = initialize_light_client(byte, init_bootstrap);
    let update = test_cases[0].updates[0].clone();
    let slot: U64 = 8906358.into();
    let genesis_validotors = Bytes32::default();

    let params = Params {
        slot,
        genesis_validotors,
        update,
        lc_store,
    };

    let file = File::open("/home/suraj3404/high/sceth/eth-light-client/test-vectors/ethereum-mainnet-slot-8905792.json")?;

    let env = ExecutorEnv::builder()
    .stdin(file).build().unwrap();

    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, LIGHT_CLIENT_ELF)?;

    receipt.verify(LIGHT_CLIENT_ID)?;

    Ok(())
}
