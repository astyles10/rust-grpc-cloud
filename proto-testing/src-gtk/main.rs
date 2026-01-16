use gtk::{
  glib::{self, clone},
  prelude::*,
  Application, ApplicationWindow,
};
use protobuf::{self, Message};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

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

fn build_ui(app: &Application) {
  let button: gtk::Button = gtk::Button::builder()
    .margin_top(12)
    .margin_bottom(12)
    .margin_end(12)
    .margin_start(12)
    .label("Open file dialog")
    .build();

  let text_view: gtk::TextView = gtk::TextView::builder().build();

  let window: ApplicationWindow = ApplicationWindow::builder()
    .application(app)
    .title("My GTK App wow")
    .child(&button)
    .default_width(350)
    .default_height(70)
    .visible(true)
    .build();

  button.connect_clicked(glib::clone!(@weak window, @weak text_view => move |_| {
    let fileopener = gtk::FileDialog::builder()
      .title("Open file or directory")
      .accept_label("Open")
      .modal(true)
      .build();
    fileopener.open(Some(&window), gio::Cancellable::NONE, move |file| {
      if let Ok(file) = file {
        let filename = file.path().expect("Couldn't get file path");
        let file = File::open(filename).expect("Couldn't open file");
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        let _ = reader.read_to_string(&mut contents);
        text_view.buffer().set_text(&contents);
      }
    })
  }));

  window.connect_close_request(move |window| {
    if let Some(application) = window.application() {
      application.remove_window(window);
    }
    glib::Propagation::Proceed
  });

  window.present();
}

fn main() -> glib::ExitCode {
  // Take in file/directory path from GUI
  // Generate descriptor files using protoc
  // Read descriptors into FileDescriptorSet
  let proto_file = fs::read_to_string(
    "/home/beefmince/Projects/rust-grpc-cloud/proto-testing/proto/proto_example/test.pb",
  )
  .expect("Failed to open file");
  let mut protoo: protobuf::descriptor::FileDescriptorSet =
    protobuf::descriptor::FileDescriptorSet::new();
  protoo
    .merge_from_bytes(proto_file.as_bytes())
    .expect("Failed to merge from file");
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

  let app: Application = Application::builder().application_id(APP_ID).build();
  app.connect_activate(build_ui);
  app.run()
}

// pub fn kek_build_ui(application: &gtk::Application) {
//     let ui_src = include_str!("text_viewer.ui");
//     let builder = gtk::Builder::from_string(ui_src);

//     let window = builder
//         .object::<gtk::ApplicationWindow>("window")
//         .expect("Couldn't get window");
//     window.set_application(Some(application));
//     let open_button = builder
//         .object::<gtk::Button>("open_button")
//         .expect("Couldn't get builder");
//     let text_view = builder
//         .object::<gtk::TextView>("text_view")
//         .expect("Couldn't get text_view");

//     open_button.connect_clicked(glib::clone!(@weak window, @weak text_view => move |_| {

//         let dialog = gtk::FileDialog::builder()
//             .title("Open File")
//             .accept_label("Open")
//             .build();

//         dialog.open(Some(&window), gio::Cancellable::NONE, move |file| {
//             if let Ok(file) = file {
//                 let filename = file.path().expect("Couldn't get file path");
//                 let file = File::open(filename).expect("Couldn't open file");

//                 let mut reader = BufReader::new(file);
//                 let mut contents = String::new();
//                 let _ = reader.read_to_string(&mut contents);

//                 text_view.buffer().set_text(&contents);
//             }
//         });
//     }));