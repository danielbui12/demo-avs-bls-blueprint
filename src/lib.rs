pub mod contexts;
pub mod jobs;
pub mod error;

use blueprint_sdk::alloy::primitives::{address, Address};
use blueprint_sdk::alloy::sol;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::LazyLock;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    TangleTaskManager,
    "contracts/out/TangleTaskManager.sol/TangleTaskManager.json"
);

pub static TASK_MANAGER_ADDRESS: LazyLock<Address> = LazyLock::new(|| {
    env::var("TASK_MANAGER_ADDRESS")
        .map(|addr| addr.parse().expect("Invalid TASK_MANAGER_ADDRESS"))
        .unwrap_or_else(|_| address!("c0f115a19107322cfbf1cdbc7ea011c19ebdb4f8"))
});

// @dev Anvil Account #3
pub static AGGREGATOR_PRIVATE_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("AGGREGATOR_PRIVATE_KEY").unwrap_or_else(|_| {
        "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6".to_string()
    })
});
pub static AGGREGATOR_ADDRESS: blueprint_sdk::alloy::primitives::Address = address!("90F79bf6EB2c4f870365E785982E1f101E93b906");

// @dev Anvil Account #4
pub static GENERATOR_PRIVATE_KEY: LazyLock<String> = LazyLock::new(|| {
    env::var("GENERATOR_PRIVATE_KEY").unwrap_or_else(|_| {
        "47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a".to_string()
    })
});
pub static GENERATOR_ADDRESS: blueprint_sdk::alloy::primitives::Address = address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65");
