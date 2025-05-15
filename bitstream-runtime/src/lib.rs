#![cfg_attr(not(feature = "std"), no_std)]

pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{Everything, Nothing},
};
pub use frame_system::EnsureRoot;
use sp_core::OpaqueMetadata;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    transaction_validity::{TransactionSource, TransactionValidity},
    generic,
    ApplyExtrinsicResult,
};

pub type BlockNumber = u32;
pub type Signature = sp_runtime::MultiSignature;
pub type AccountId = <<Signature as sp_runtime::traits::Verify>::Signer as sp_runtime::traits::IdentifyAccount>::AccountId;
pub type Index = u32;
pub type Hash = sp_core::H256;
pub type Balance = u128;

parameter_types! {
    pub const BlockHashCount: u32 = 2400;
}

impl frame_system::Config for Runtime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = Index;
    type Call = Call;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system,
        Balances: pallet_balances,
        // Pallets will be injected here next
    }
);
