use protobuf_codegen::Codegen;

fn main() {
  Codegen::new()
    .cargo_out_dir("protos")
    .include("proto")
    .inputs(["proto/proto_example/foo.proto", "proto/proto_example/bar/bar.proto"])
    .run_from_script();
}
