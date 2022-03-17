use std::error::Error;
use std::fs::File;
use std::fs;
use state_fold_types::contract;

macro_rules! rerun_if_changed {
    ($path:expr) => {
        println!("cargo:rerun-if-changed={}", $path);
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    rerun_if_changed!("build.rs");

    let source = fs::canonicalize("src/contracts/bin/SimpleStorage.json")?;
    rerun_if_changed!(source.to_str().unwrap());
    let source = File::open(source)?;

    let output = contract::path!("simple_storage.rs");
    rerun_if_changed!(output);
    let output = File::create(output)?;

    contract::write("SimpleStorage", source, output)?;

    Ok(())
}
