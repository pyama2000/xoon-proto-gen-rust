fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("kintai_service_descriptor.bin"))
        .compile(
            &["xoon-proto/proto/kintai/v1/kintai_service.proto"],
            &["xoon-proto/proto"],
        )?;
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("notification_service_descriptor.bin"))
        .compile(
            &["xoon-proto/proto/notification/v1/notification_service.proto"],
            &["xoon-proto/proto"],
        )?;
    Ok(())
}
