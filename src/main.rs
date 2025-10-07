use test_eigen_bls_blueprint as blueprint;

use blueprint::TangleTaskManager;
use blueprint::{PRIVATE_KEY, TASK_MANAGER_ADDRESS};
use blueprint_sdk::alloy::network::EthereumWallet;
use blueprint_sdk::alloy::primitives::{address, Address, Bytes};
use blueprint_sdk::alloy::signers::local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::testing::utils::setup_log;
use blueprint_sdk::{info, tokio, warn, Router};
use std::sync::Arc;
use std::time::Duration;

// TODO: Replace with your context name
use blueprint::ExampleContext;
use blueprint::{example_task, EXAMPLE_JOB_ID};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();
    let env = BlueprintEnvironment::load()?;
    let signer: PrivateKeySigner = PRIVATE_KEY.parse().expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(signer);

    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());

    let client = Arc::new(provider.clone());
    let task_producer = PollingProducer::new(
        client.clone(),
        PollingConfig::default().poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;
    // TODO: Replace with your context name
    let context = ExampleContext {
        config: env.clone(),
    };

    // Create an instance of your task manager
    let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider);

    // Spawn a task to create a task - this is just for testing/example purposes
    info!("Spawning a task to create a task on the contract...");
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            // @dev We use the Anvil Account #4 as the Task generator address
            let task = contract
                .createNewTask(Bytes::from_static(b"World"), 100u32, vec![0].into())
                .from(address!("15d34AAf54267DB7D7c367839AAf71A00a2C6A65"));
            let receipt = task.send().await.unwrap().get_receipt().await.unwrap();
            if receipt.status() {
                info!("Task created successfully");
            } else {
                warn!("Task creation failed");
            }
        }
    });

    // // Prevent main from exiting by awaiting a pending future
    // tokio::signal::ctrl_c().await.expect("failed to listen for ctrl_c");

    info!("Starting the event watcher ...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::builder(eigen_config, env)
        .router(
            // TODO: Update your task
            Router::new()
                .route(EXAMPLE_JOB_ID, example_task)
                .with_context(context),
        )
        .producer(task_producer)
        .with_shutdown_handler(async {
            info!("Shutting down task manager service");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
