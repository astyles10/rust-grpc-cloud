// use protobuf::text_format::print_to_string;
use protobuf::{self, Message};
use std::os::unix::process::{CommandExt};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::{env, fs, vec};

const DESCRIPTOR_OUTPUT_PATH: &str = "/tmp/pathfinder";

fn make_path(file_path: String) -> PathBuf {
  let absolute_path = Command::new("realpath")
    .arg(file_path)
    .output()
    .expect("Error making path");
  let absolute_path = match str::from_utf8(&absolute_path.stdout) {
    Ok(v) => v.trim(),
    Err(e) => {
      println!("{:?}", e);
      ""
    }
  };
  PathBuf::from(absolute_path)
}

fn parse_command_line_args(mut args: Vec<String>) -> Vec<String> {
  let mut proto_dirs: Vec<String> = vec![String::from(""); args.len() - 1];
  let (_, right) = args.split_at_mut(0);
  proto_dirs.clone_from_slice(&right[1..right.len()]);
  proto_dirs
}

fn find_protos_by_dir(directory: &Path) -> Vec<PathBuf> {
  let command: Result<std::process::Output, std::io::Error> = Command::new("find")
    .arg(directory)
    .args(["-type", "f"])
    .args(["-name", "*.proto"])
    .output();
  let mut proto_paths: Vec<PathBuf> = Vec::new();
  match command {
    Ok(cmd_output) => {
      // TODO: Come back and clean this up, this is atrocious
      if let Some(val) = cmd_output.status.code() {
        if val == 0 {
          let proto_files: &str = str::from_utf8(&cmd_output.stdout).expect("Something failed");
          let proto_files: std::str::Lines<'_> = proto_files.lines();
          for file_path in proto_files {
            proto_paths.push(PathBuf::from(file_path));
          }
        } else {
          println!(
            "stderr: {}",
            str::from_utf8(&cmd_output.stderr).expect("Failed to read stderr")
          );
        }
      } else {
        println!(
          "Find output = {}",
          str::from_utf8(&cmd_output.stdout).expect("Something failed")
        );
      }
    }
    Err(e) => {
      println!("{e}");
    }
  }
  proto_paths
}

fn generate_proto_desciptors(proto_include_path: &PathBuf, proto_file_paths: Vec<PathBuf>) {
  // protoc -I ./proto <proto-file-path> -o <proto-file-descriptor>.pb --include_imports --include_source_info
  // let mut generated_descriptors: Vec<PathBuf> = Vec::new();
  for mut proto_file_path in proto_file_paths {
    let mut output_file_name = proto_file_path.clone();
    output_file_name.set_extension("pb");
    let output_file_name = output_file_name.file_name();
    if let Some(file_name) = output_file_name {
      let file_name: std::borrow::Cow<'_, str> = file_name.to_string_lossy();
      let output_path = format!("{DESCRIPTOR_OUTPUT_PATH}/{file_name}");
      let proto_descriptor_command: Result<ExitStatus, std::io::Error> = Command::new("protoc")
        .arg("-I")
        .arg(proto_include_path)
        .arg(&proto_file_path)
        .args(["-o", output_path.as_str()])
        .args(["--include_imports", "--include_source_info"])
        .status();
      match proto_descriptor_command {
        // Does not output anything, just need to check the return code
        Ok(exit_code) => {
          if exit_code.success() {
            let proto_file_result: Result<String, std::io::Error> =
              std::fs::read_to_string(&output_path);
            match proto_file_result {
              Ok(descriptor_file_contents) => {
                let mut proto_fd: protobuf::descriptor::FileDescriptorSet =
                  protobuf::descriptor::FileDescriptorSet::new();
                proto_fd
                  .merge_from_bytes(descriptor_file_contents.as_bytes())
                  .expect("Failed to parse proto file");
                println!("{:?}", proto_fd);
              }
              Err(e) => {
                println!("Failed to read file {output_path}: {e}");
              }
            }
          } else {
            println!(
              "protoc failed with exit code {} for {:?}",
              exit_code,
              &proto_file_path.as_mut_os_str()
            );
          }
        }
        Err(exec_error) => {
          println!("{}", exec_error);
        }
      }
    }
  }
}

fn main() {
  let make_tmp_dir = Command::new("mkdir")
    .args(["-p", DESCRIPTOR_OUTPUT_PATH])
    .output();
  println!("{}", str::from_utf8(&make_tmp_dir.expect("Failed to make temp directory").stdout).expect(""));
  // Take in directory(s) from command line
  // let args: Vec<String> = env::args().collect();
  // let proto_dirs = parse_command_line_args(args);
  // for dir in proto_dirs {
  //   let path = make_path(dir);
  //   // Find all protos in dir
  //   let proto_file_paths: Vec<PathBuf> = find_protos_by_dir(&path);
  //   generate_proto_desciptors(&path, proto_file_paths);
  //   // Create file descriptors for each file
  // }

  // Take in file/directory path from GUI
  let proto_file = fs::read_to_string(
    "/tmp/pathfinder/test.pb",
  )
  .expect("Failed to open file");
  let mut protoo: protobuf::descriptor::FileDescriptorSet =
    protobuf::descriptor::FileDescriptorSet::new();
  protoo
    .merge_from_bytes(proto_file.as_bytes())
    .expect("Failed to merge from file");
  println!("{:?}", protoo);
}
