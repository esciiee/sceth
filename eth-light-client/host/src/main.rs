use methods::{LIGHT_CLIENT_ELF, LIGHT_CLIENT_ID};
use near_zk_types::{
    LightClientBlockLiteView, LightClientBlockView, PrevBlockContext, ValidatorStakeView,
};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use serde::{Deserialize, Serialize};
use eth_lc::{initialize_light_client, types::Bytes32, LightClientBootstrap, LightClientStore};

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
}

fn main() -> anyhow::Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // NOTE: These test vectors come from https://github.com/austinabell/near-light-client-tests
    //       and are generated using mainnet data pulled from RPC.
    // let contents = include_str!("../../test-vectors/mainnet-80000000-81000000.json");
    let contents = include_str!("../../test-vectors/ethereum-mainnet-slot-8905792.json");
    let test_cases: Vec<TestCase> = serde_json::from_str(&contents)?;
    let init_bootstrap = &test_cases[0].init;
    let byte: Bytes32 = Bytes32::default();
    let lc_store = initialize_light_client(byte, init_bootstrap);

    Ok(())
}
