pub mod kintai_service {
    tonic::include_proto!("kintai.v1");
    pub const KINTAI_SERVICE_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("kintai_service_descriptor");
}
