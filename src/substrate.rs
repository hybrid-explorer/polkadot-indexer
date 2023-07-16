use hybrid_indexer::{shared::RuntimeIndexer, substrate::Indexer};

use crate::polkadot::runtime_types::{
    pallet_collective::pallet::{Event as CollectiveEvent, Event2 as CollectiveEvent2},
    pallet_democracy::pallet::Event as DemocracyEvent,
    pallet_elections_phragmen::pallet::Event as ElectionsPhragmenEvent,
};

pub fn democracy_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: DemocracyEvent,
) {
    match event {
        DemocracyEvent::Proposed { proposal_index, .. } => {
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
        }
        DemocracyEvent::Tabled { proposal_index, .. } => {
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
        }
        DemocracyEvent::ExternalTabled => {}
        DemocracyEvent::Started { ref_index, .. } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Passed { ref_index } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::NotPassed { ref_index } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Cancelled { ref_index } => {
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Delegated { who, target } => {
            indexer.index_event_account_id(who, block_number, event_index);
            indexer.index_event_account_id(target, block_number, event_index);
        }
        DemocracyEvent::Undelegated { account } => {
            indexer.index_event_account_id(account, block_number, event_index);
        }
        DemocracyEvent::Vetoed {
            who, proposal_hash, ..
        } => {
            indexer.index_event_account_id(who, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        DemocracyEvent::Blacklisted { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        DemocracyEvent::Voted {
            voter, ref_index, ..
        } => {
            indexer.index_event_account_id(voter, block_number, event_index);
            indexer.index_event_ref_index(ref_index, block_number, event_index);
        }
        DemocracyEvent::Seconded {
            seconder,
            prop_index,
        } => {
            indexer.index_event_account_id(seconder, block_number, event_index);
            indexer.index_event_proposal_index(prop_index, block_number, event_index);
        }
        DemocracyEvent::ProposalCanceled { prop_index } => {
            indexer.index_event_proposal_index(prop_index, block_number, event_index);
        }
        DemocracyEvent::MetadataSet { owner: _, hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        DemocracyEvent::MetadataCleared { owner: _, hash } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
        DemocracyEvent::MetadataTransferred {
            prev_owner: _,
            owner: _,
            hash,
        } => {
            indexer.index_event_preimage_hash(hash.into(), block_number, event_index);
        }
    }
}

pub fn collective_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: CollectiveEvent,
) {
    match event {
        CollectiveEvent::Proposed {
            account,
            proposal_index,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Voted {
            account,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Approved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Disapproved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Executed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::MemberExecuted { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent::Closed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
    }
}

pub fn collective2_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: CollectiveEvent2,
) {
    match event {
        CollectiveEvent2::Proposed {
            account,
            proposal_index,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_index(proposal_index, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Voted {
            account,
            proposal_hash,
            ..
        } => {
            indexer.index_event_account_id(account, block_number, event_index);
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Approved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Disapproved { proposal_hash } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Executed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::MemberExecuted { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
        CollectiveEvent2::Closed { proposal_hash, .. } => {
            indexer.index_event_proposal_hash(proposal_hash.into(), block_number, event_index);
        }
    }
}

pub fn elections_phragmen_index_event<R: RuntimeIndexer>(
    indexer: &Indexer<R>,
    block_number: u32,
    event_index: u32,
    event: ElectionsPhragmenEvent,
) {
    match event {
        ElectionsPhragmenEvent::NewTerm { new_members } => {
            for member in &new_members {
                indexer.index_event_account_id(member.0.clone(), block_number, event_index);
            }
        }
        ElectionsPhragmenEvent::EmptyTerm => {}
        ElectionsPhragmenEvent::ElectionError => {}
        ElectionsPhragmenEvent::MemberKicked { member } => {
            indexer.index_event_account_id(member, block_number, event_index);
        }
        ElectionsPhragmenEvent::Renounced { candidate } => {
            indexer.index_event_account_id(candidate, block_number, event_index);
        }
        ElectionsPhragmenEvent::CandidateSlashed { candidate, .. } => {
            indexer.index_event_account_id(candidate, block_number, event_index);
        }
        ElectionsPhragmenEvent::SeatHolderSlashed { seat_holder, .. } => {
            indexer.index_event_account_id(seat_holder, block_number, event_index);
        }
    }
}
