fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(cfg!(feature = "client"))
        .build_server(cfg!(feature = "server"))
        .compile(
            &["../grpc-interfaces/state-fold-server.proto"],
            &["../grpc-interfaces/"],
        )?;

    println!("cargo:rerun-if-changed=../grpc-interfaces/state-fold-server.proto");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
