fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(&["./proto/unary_server.proto"], &["./proto"])
        .unwrap();
}
