
# Nerd GTK Icons

A library for embedding Nerd Fonts SVG icons as GResource in GTK4/Adwaita Rust applications.

## Installation

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
nerd_gtk_icons = { path = "../nerd_gtk_icons" }
```


2. Register the resources and add the icon theme path in your application (see example below):

```rust
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
    // ... your UI code ...
}
```

3. Use icons by name:

```rust
let image = gtk::Image::from_icon_nam(nerd_gtk_icons::icons::NF_DEV_RUST_SYMBOLIC);
```

## Example

See the full example in [examples/adw_demo.rs](examples/adw_demo.rs).

---

- Make sure `build.rs` generated and included the resources.
- Icon names are available via the `nerd_gtk_icons::icons` module.
