use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, glib};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
  let app: Application = Application::builder().application_id(APP_ID).build();
  app.connect_activate(build_ui);
  app.run()
}

fn build_ui(app: &Application) {
  let button: gtk::Button = gtk::Button::builder()
    .margin_top(12)
    .margin_bottom(12)
    .margin_end(12)
    .margin_start(12)
    .build();

  button.connect_clicked(|button| {
    button.set_label("Hello worl");
  });

  let window: ApplicationWindow = ApplicationWindow::builder()
      .application(app)
      .title("My GTK App wow")
      .child(&button)
      .build();
  window.present();
}
