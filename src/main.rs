use test_eigen_bls_blueprint as blueprint;

use blueprint_sdk::alloy::network::EthereumWallet;
use blueprint_sdk::alloy::primitives::{Address, Bytes};
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

use blueprint::TangleTaskManager;
use blueprint::{AGGREGATOR_PRIVATE_KEY, GENERATOR_PRIVATE_KEY, GENERATOR_ADDRESS, TASK_MANAGER_ADDRESS};
use blueprint::contexts::combined::CombinedContext;
use blueprint::contexts::client::AggregatorClient;
use blueprint::contexts::aggregator::AggregatorContext;
use blueprint::jobs::initialize_task::{initialize_bls_task, INITIALIZE_TASK_JOB_ID};
// TODO: Replace with your context name
use blueprint::contexts::example_context::ExampleContext;
use blueprint::jobs::example_task::{example_task, EXAMPLE_JOB_ID};

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    setup_log();
    let env = BlueprintEnvironment::load()?;
    let aggregator_signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY.parse().expect("failed to generate wallet ");
    let wallet = EthereumWallet::from(aggregator_signer);

    let http_rpc_endpoint = env.http_rpc_endpoint.clone();
    let provider = get_wallet_provider_http(http_rpc_endpoint.clone(), wallet.clone());
    let server_address = format!("{}:{}", "127.0.0.1", 8081);

    let context = ExampleContext {
        client: AggregatorClient::new(&server_address)
            .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?,
        std_config: env.clone(),
    };

    let aggregator_context = AggregatorContext::new(
        server_address,
        *TASK_MANAGER_ADDRESS,
        wallet.clone(),
        env.clone(),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;


    // Create the combined context for both tasks
    let combined_context = CombinedContext::new(
        context,
        Some(aggregator_context.clone()),
        env.clone(),
    );
    let client = Arc::new(provider);

    // Create producer for task events
    let task_producer = PollingProducer::new(
        client.clone(),
        // PollingConfig::default().poll_interval(Duration::from_secs(1)),
        PollingConfig::from_current().step(1).confirmations(1u64).poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    // // Spawn a task to create a task - this is just for testing/example purposes
    // info!("Spawning a task to create a task on the contract...");
    // tokio::spawn(async move {
    //     let generator_signer: PrivateKeySigner = GENERATOR_PRIVATE_KEY.parse().expect("failed to generate wallet ");
    //     let wallet = EthereumWallet::from(generator_signer);
    //     let provider = get_wallet_provider_http(http_rpc_endpoint, wallet.clone());
    //     let contract = TangleTaskManager::new(*TASK_MANAGER_ADDRESS, provider.clone());
    //     loop {
    //         tokio::time::sleep(Duration::from_secs(5)).await;
    //         let task = contract
    //             .createNewTask(Bytes::from_static(b"World"), 100u32, vec![0].into())
    //             .from(GENERATOR_ADDRESS);
    //         let receipt = task.send().await.unwrap().get_receipt().await.unwrap();
    //         if receipt.status() {
    //             info!("Task created successfully");
    //         } else {
    //             warn!("Task creation failed");
    //         }
    //     }
    // });

    info!("Starting the event watcher ...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default())
        .with_exit_after_register(false);
    BlueprintRunner::builder(eigen_config, env)
        .router(
            // TODO: Update your task
            Router::new()
                .route(EXAMPLE_JOB_ID, example_task)
                .route(INITIALIZE_TASK_JOB_ID, initialize_bls_task)
                .with_context(combined_context),
        )
        .producer(task_producer)
        .background_service(aggregator_context)
        .with_shutdown_handler(async {
            info!("Shutting down task manager service");
        })
        .run()
        .await?;

    info!("Exiting...");
    Ok(())
}
