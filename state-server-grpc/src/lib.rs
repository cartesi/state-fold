use tokio::signal;
use tokio::sync::oneshot;
use tonic::transport::Server;

use crate::state_server::delegate_manager_server::{
    DelegateManager, DelegateManagerServer,
};

pub mod state_server {
    tonic::include_proto!("state_server");
}

pub async fn serve_delegate_manager<T: DelegateManager>(
    address: &str,
    manager: T,
    kill_switch: oneshot::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_server) =
        tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<DelegateManagerServer<T>>()
        .await;

    let addr = address.parse().unwrap();

    println!("StateFoldServer listening on {}", addr);

    Server::builder()
        .add_service(health_server)
        .add_service(DelegateManagerServer::new(manager))
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
