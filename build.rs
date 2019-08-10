fn main() {
  tower_grpc_build::Config::new()
    .enable_server(true)
    .enable_client(true)
    .build(
      &["proto/auditer/auditer.proto"],
      &["proto/auditer"]
    )
    .unwrap_or_else(|err| panic!("protobuf compilation failed: {}", err));
  println!("cargo:rerun-if-changed=proto/auditer/auditer.proto");
}
