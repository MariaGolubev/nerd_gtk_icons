use adw::gtk;
use adw::prelude::*;
use nerd_gtk_icons;

fn main() {
    nerd_gtk_icons::register_icons().expect("Failed to load icons");

    let app = adw::Application::builder()
        .application_id("com.example.NerdGtkIconsDemo")
        .build();

    app.connect_startup(startup);
    app.connect_activate(build_ui);
    app.run();
}

fn startup(_app: &adw::Application) {
    let display = gtk::gdk::Display::default().expect("No display");
    let theme = gtk::IconTheme::for_display(&display);

    theme.add_resource_path(nerd_gtk_icons::ICONS_RESOURCE_PATH);
}

fn build_ui(app: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("Nerd GTK Icons Example")
        .default_width(600)
        .default_height(400)
        .build();

    let header_bar = adw::HeaderBar::builder().show_title(false).build();
    header_bar.add_css_class("flat");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    vbox.append(&header_bar);

    let status_page = adw::StatusPage::builder()
        .vexpand(true)
        .icon_name(nerd_gtk_icons::icons::NF_DEV_RUST_SYMBOLIC)
        .title("Nerd GTK Icons")
        .description("An example of using Nerd GTK Icons in an Adwaita application")
        .build();

    vbox.append(&status_page);

    window.set_content(Some(&vbox));
    window.present();
}
