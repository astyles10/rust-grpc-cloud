use protobuf::{self, Message};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs};

mod hello {
  include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
}

fn get_proto_file_descriptors(proto_dir: &Path) -> protobuf::descriptor::FileDescriptorSet {
  compile_proto_descriptors(proto_dir);
  protobuf::descriptor::FileDescriptorSet::new()
}

fn compile_proto_descriptors(proto_dir: &Path) -> &Path {
  // Execute script using proto_dir path which outputs to /tmp
  // https://doc.rust-lang.org/std/process/struct.Command.html
  proto_dir
}

fn make_path(file_path: String) -> PathBuf {
  PathBuf::from(file_path)
}

fn main () {
  // Take in file/directory path from GUI
  // Generate descriptor files using protoc
  // Read descriptors into FileDescriptorSet
  let proto_file = fs::read_to_string("/home/beefmince/Projects/rust-grpc-cloud/proto-testing/proto/proto_example/test.pb")
    .expect("Failed to open file");
  let mut protoo: protobuf::descriptor::FileDescriptorSet = protobuf::descriptor::FileDescriptorSet::new();
  protoo.merge_from_bytes(proto_file.as_bytes()).expect("Failed to merge from file");
  println!("{:?}", protoo);
  let data = r#"{
    "int": 69,
    "kewk": "rabble"
  }"#;
  let fruit: hello::bar::Bar = protobuf_json_mapping::parse_from_str(data).unwrap();
  println!("{:?}", fruit);
  // let test_descriptor = protobuf::reflect::FileDescriptor::new_dynamic(desc_proto, dependencies);
  let filedescriptor: protobuf::reflect::FileDescriptor = hello::foo::file_descriptor().clone();
  println!("{:?}", filedescriptor);
}