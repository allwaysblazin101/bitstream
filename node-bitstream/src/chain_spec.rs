use sc_service::ChainType;
use sp_core::sr25519;
use sp_runtime::traits::IdentifyAccount;
use bitstream_runtime::{AccountId, GenesisConfig};

pub fn development_config() -> sc_service::GenericChainSpec<GenesisConfig> {
    sc_service::GenericChainSpec::from_genesis(
        "Bitstream Dev",
        "bitstream_dev",
        ChainType::Development,
        move || testnet_genesis(),
        vec![],
        None,
        None,
        None,
        None,
    )
}

fn testnet_genesis() -> GenesisConfig {
    GenesisConfig {
        system: Default::default(),
        balances: Default::default(),
        sudo: Default::default(),
        pose: Default::default(),
        token_launch: Default::default(),
        forum: Default::default(),
        dao: Default::default(),
        treasury: Default::default(),
    }
}
