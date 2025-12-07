// use protobuf::descriptor::field_descriptor_proto::Type;
// use protobuf::reflect::FieldDescriptor;
// use protobuf::reflect::MessageDescriptor;
use protobuf_codegen::Codegen;
// use protobuf_codegen::Customize;
// use protobuf_codegen::CustomizeCallback;

fn main() {
  Codegen::new()
    .cargo_out_dir("protos")
    // .include("proto")
    // .inputs(["proto/proto_example/foo.proto", "proto/proto_example/bar/bar.proto"])
    .include("proto")
    .inputs(["proto/proto_example/foo.proto", "proto/proto_example/bar/bar.proto"])
    // .customize_callback(GenSerde)
    .run_from_script();
}
