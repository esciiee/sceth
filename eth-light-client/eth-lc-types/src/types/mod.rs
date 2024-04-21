use ssz_rs::{prelude::*, Vector};

use self::primitives::{ByteList, ByteVector, U64};

pub mod primitives;

pub type Address = ByteVector<20>;
pub type Bytes32 = ByteVector<32>;
pub type LogsBloom = ByteVector<256>;
pub type BLSPubKey = ByteVector<48>;
pub type SignatureBytes = ByteVector<96>;
pub type Transaction = ByteList<1073741824>;

#[derive(serde::Deserialize, Debug, Default, SimpleSerialize, Clone)]
pub struct BeaconBlockHeader {
    pub slot: U64,
    pub proposer_index: U64,
    pub parent_root: Bytes32,
    pub state_root: Bytes32,
    pub body_root: Bytes32,
}

#[derive(Debug, Clone, Default, SimpleSerialize, serde::Deserialize)]
pub struct SyncCommittee {
    pub pubkeys: Vector<BLSPubKey, 512>,
    pub aggregate_pubkey: BLSPubKey,
}

#[derive(serde::Deserialize, Debug, Clone, Default, SimpleSerialize)]
pub struct SyncAggregate {
    pub sync_committee_bits: Bitvector<512>,
    pub sync_committee_signature: SignatureBytes,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Update {
    // #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: BeaconBlockHeader,
    pub next_sync_committee: SyncCommittee,
    pub next_sync_committee_branch: Vec<Bytes32>,
    // #[serde(deserialize_with = "header_deserialize")]
    pub finalized_header: BeaconBlockHeader,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

pub struct GenericUpdate {
    pub attested_header: BeaconBlockHeader,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: u64,
    pub next_sync_committee: Option<SyncCommittee>,
    pub next_sync_committee_branch: Option<Vec<Bytes32>>,
    pub finalized_header: Option<BeaconBlockHeader>,
    pub finality_branch: Option<Vec<Bytes32>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct FinalityUpdate {
    // #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: BeaconBlockHeader,
    // #[serde(deserialize_with = "header_deserialize")]
    pub finalized_header: BeaconBlockHeader,
    pub finality_branch: Vec<Bytes32>,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

#[derive(serde::Deserialize, Debug)]
pub struct OptimisticUpdate {
    // #[serde(deserialize_with = "header_deserialize")]
    pub attested_header: BeaconBlockHeader,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: U64,
}

impl From<&Update> for GenericUpdate {
    fn from(update: &Update) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: Some(update.next_sync_committee.clone()),
            next_sync_committee_branch: Some(update.next_sync_committee_branch.clone()),
            finalized_header: Some(update.finalized_header.clone()),
            finality_branch: Some(update.finality_branch.clone()),
        }
    }
}

impl From<&FinalityUpdate> for GenericUpdate {
    fn from(update: &FinalityUpdate) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: Some(update.finalized_header.clone()),
            finality_branch: Some(update.finality_branch.clone()),
        }
    }
}

impl From<&OptimisticUpdate> for GenericUpdate {
    fn from(update: &OptimisticUpdate) -> Self {
        Self {
            attested_header: update.attested_header.clone(),
            sync_aggregate: update.sync_aggregate.clone(),
            signature_slot: update.signature_slot.into(),
            next_sync_committee: None,
            next_sync_committee_branch: None,
            finalized_header: None,
            finality_branch: None,
        }
    }
}
