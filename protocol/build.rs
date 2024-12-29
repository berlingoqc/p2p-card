extern crate prost_build;

fn main() {
    let proto_files = &["proto/client.proto"]; // List your .proto files here
    let proto_include_dirs = &["proto/"];       // Directory containing the .proto files

    prost_build::Config::new()
        .out_dir("src/") // Directory for generated code
        .compile_protos(proto_files, proto_include_dirs)
        .expect("Failed to compile Protobuf files");
}