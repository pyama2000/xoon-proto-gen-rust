fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("xoon-proto/proto/kintai/v1/kintai_service.proto")?;
    Ok(())
}
