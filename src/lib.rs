pub mod kintai_service {
    tonic::include_proto!("kintai.v1");
    pub const KINTAI_SERVICE_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("kintai_service_descriptor");
}

pub mod notification_service {
    tonic::include_proto!("notification.v1");
    pub const NOTIFICATION_SERVICE_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("notification_service_descriptor");
}

pub mod automation {
    pub mod v1 {
        tonic::include_proto!("automation.v1");
        pub const RELEASE_SERVICE_FILE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("release_service_descriptor");
    }
}

pub mod server_status {
    pub mod v1 {
        tonic::include_proto!("server_status.v1");
        pub const SERVER_STATUS_SERVICE_DESCRIPTOR_SET: &[u8] =
            tonic::include_file_descriptor_set!("server_status_service_descriptor");
    }
}
