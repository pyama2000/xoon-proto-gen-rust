fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("xoon-proto/proto/retty/sample/sample.proto")?;
    Ok(())
}
