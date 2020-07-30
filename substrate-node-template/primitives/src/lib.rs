#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use frame_support::dispatch;
use codec::{Encode, Decode};

pub trait Vpp<AccountId> {
    fn update_status(who: &AccountId, idx: u64, approval_status: ApprovalStatus) -> dispatch::DispatchResult;
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, PartialEq, Eq, Clone, Copy)]
pub enum ApprovalStatus {
    Denied,
    Passed,
    Pending,
}
