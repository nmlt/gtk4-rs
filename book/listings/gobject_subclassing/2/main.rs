mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::new(Some("org.gtk.example"), Default::default());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
// ANCHOR: activate
fn build_ui(application: &Application) {
    // Create a window
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .build();

    // Create a button
    let button = CustomButton::new();
    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

    // Add button
    window.set_child(Some(&button));
    window.present();
}
// ANCHOR_END: activate
