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
        PoSE: pallet_pose,
        TokenLaunch: pallet_token_launch,
        Forum: pallet_forum,
        Treasury: pallet_treasury,
        DAO: pallet_dao,
        Content: pallet_content,
        // Pallets will be injected here next
    }
);
impl pallet_pose::Config for Runtime {}
impl pallet_token_launch::Config for Runtime {
    type Currency = Balances;
    type TreasuryAccount = ();
}
impl pallet_forum::Config for Runtime {}
impl pallet_treasury::Config for Runtime {
    type Currency = Balances;
    type BurnPercent = frame_support::traits::ConstU8<33>;
    type DiversifyPercent = frame_support::traits::ConstU8<33>;
    type DAOAccount = ();
}
impl pallet_dao::Config for Runtime {}
impl pallet_content::Config for Runtime {}
