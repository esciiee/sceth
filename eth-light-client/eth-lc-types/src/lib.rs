use types::{primitives::*, BeaconBlockHeader};
use ssz_rs::prelude::*;
use crate::types::*;
use eyre::Result;

pub mod types;

#[derive(Debug, serde::Deserialize)]
pub struct LightClientStore {
    pub finalized_header: LCHeader,
    pub current_sync_committee: SyncCommittee,
    pub next_sync_committee: Option<SyncCommittee>,
    pub optimistic_header: LCHeader,
    pub previous_max_active_participants: u64,
    pub current_max_active_participants: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct LightClientBootstrap {
    pub header: LCHeader,
    pub current_sync_committee: SyncCommittee,
    pub current_sync_committee_branch: Vec<Bytes32>
}

// the only job here is to verify the sigs. lets be optimistic about the code inputs for now
pub fn generic_update(
    store: LightClientStore,
    update: GenericUpdate,
    current_slot: u64,
    genesis_validator_root: Bytes32,
) -> eyre::Result<()> {
    let bits = get_bits(&update.sync_aggregate.sync_committee_bits);
    if bits == 0 {
        return Err(eyre::eyre!("No bits set in sync_committee_bits"));
    }

    let update_finalized_slot = update.finalized_header.clone().unwrap_or_default().slot;
    let valid_time = current_slot >= update.signature_slot && update.signature_slot > update.attested_header.slot.as_u64()
        && update.attested_header.slot >= update_finalized_slot;

    if !valid_time {
        return Err(eyre::eyre!("Invalid time"));
    }





    return Ok(());
}


fn get_bits(bitfield: &Bitvector<512>) -> u64 {
    let mut count = 0;
    bitfield.iter().for_each(|bit| {
        if bit == true {
            count += 1;
        }
    });

    count
}

pub fn initialize_light_client(_trusted_block_root: Bytes32, bootstrap : &LightClientBootstrap) -> LightClientStore {
    // make checks here
    LightClientStore {
        finalized_header: bootstrap.header.clone(),
        current_sync_committee: bootstrap.current_sync_committee.clone(),
        next_sync_committee: None,
        optimistic_header: bootstrap.header.clone(),
        previous_max_active_participants: 0,
        current_max_active_participants: 0,
    }
}
