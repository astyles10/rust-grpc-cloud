use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
  let app: Application = Application::builder().application_id(APP_ID).build();
  app.connect_activate(build_ui);
  app.run()
}

fn build_ui(app: &Application) {
  let window: ApplicationWindow = ApplicationWindow::builder()
      .application(app)
      .title("My GTK App wow")
      .build();
  window.present();
}
