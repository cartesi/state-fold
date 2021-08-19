fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("------> Compiling proto file");
    tonic_build::compile_protos("../grpc-interfaces/stateserver.proto")?;
    Ok(())
}
