use crate::grpc_server::StateServer;

use state_fold_server::state_fold_server::StateFoldServer;
use state_server_common::state_fold_server;

use state_fold::Foldable;

use ethers::providers::{Middleware, PubsubClient};
use state_fold_types::ethers;
use state_fold_types;

use tokio::signal;
use tokio::sync::oneshot;
use tonic::transport::Server;

pub async fn serve_delegate_manager<
    M: Middleware + 'static,
    UD: Send + Sync + 'static,
    F: Foldable<UserData = UD> + 'static,
>(
    address: &str,
    state_server: StateServer<M, UD, F>,
    kill_switch: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>>
where
    <M as Middleware>::Provider: PubsubClient,
    F::InitialState: serde::de::DeserializeOwned + 'static,
    F: serde::Serialize,
{
    let (mut health_reporter, health_server) =
        tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<StateFoldServer<StateServer<M, UD, F>>>()
        .await;

    let addr = address.parse().unwrap();

    println!("StateFoldServer listening on {}", addr);

    Server::builder()
        .add_service(health_server)
        .add_service(StateFoldServer::new(state_server))
        .serve_with_shutdown(addr, async {
            kill_switch.await.ok();
            println!("Graceful context shutdown");
        })
        .await?;

    Ok(())
}

pub async fn wait_for_signal(tx: oneshot::Sender<()>) {
    let _ = signal::ctrl_c().await;
    println!("SIGINT received: shutting down");
    let _ = tx.send(());
}
