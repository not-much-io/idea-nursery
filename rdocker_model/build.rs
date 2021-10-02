fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Check hash of file to decide if any need to rebuild
    tonic_build::configure()
        .out_dir("src")
        .compile(&["proto/rdocker.proto"], &["proto"])?;
    Ok(())
}
