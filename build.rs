fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/vector_service.proto"], &["proto"])
        .unwrap();
}
