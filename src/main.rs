use {{project-name | snake_case}} as blueprint;

use blueprint::{AGGREGATOR_PRIVATE_KEY, TASK_MANAGER_ADDRESS};
use std::sync::Arc;
use std::time::Duration;
use alloy_network::EthereumWallet;
use alloy_primitives::Address;
use alloy_signer_local::PrivateKeySigner;
use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
use blueprint_sdk::evm::util::get_wallet_provider_http;
use blueprint_sdk::runner::BlueprintRunner;
use blueprint_sdk::runner::config::BlueprintEnvironment;
use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
use blueprint_sdk::{Router, info};

use crate::contexts::aggregator::AggregatorContext;
use crate::contexts::client::AggregatorClient;
use crate::contexts::combined::CombinedContext;
// TODO: Replace with your context name
use crate::contexts::example_context::ExampleContext;

#[tokio::main]
async fn main() -> Result<(), blueprint_sdk::Error> {
    let env = BlueprintEnvironment::load()?;

    let signer: PrivateKeySigner = AGGREGATOR_PRIVATE_KEY
        .parse()
        .expect("failed to generate wallet ")
    let wallet = EthereumWallet::from(signer);
    let provider = get_wallet_provider_http(env.http_rpc_endpoint.clone(), wallet.clone());
    let server_address = format!("{}:{}", "127.0.0.1", 8081);

    // TODO: Replace with your context name
    let context = ExampleContext {
        client: AggregatorClient::new(&server_address)
            .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?,
        std_config: env.clone(),
    };

    // Create the aggregator context
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
        PollingConfig::default().poll_interval(Duration::from_secs(1)),
    )
    .await
    .map_err(|e| blueprint_sdk::Error::Other(e.to_string()))?;

    info!("Spawning a task to create a task on the contract...");
    let eigen_config = EigenlayerBLSConfig::new(Address::default(), Address::default());
    BlueprintRunner::builder(eigen_config, BlueprintEnvironment::default())
    .router(
        Router::new()
            .route(EXAMPLE_JOB_ID, xsquare_eigen)
            .route(INITIALIZE_TASK_JOB_ID, initialize_bls_task)
            .with_context(combined_context),
    )
    .producer(task_producer)
    .background_service(aggregator_context)
    .with_shutdown_handler(async {
        blueprint_sdk::info!("Shutting down task manager service");
    })
    .run()
    .await?;

    info!("Exiting...");
    Ok(())
}
