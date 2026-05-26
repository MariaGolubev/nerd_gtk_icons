/// Registers embedded Nerd Font icons as a GResource.
///
/// This function must be called once before using any icons provided by this crate
/// through GTK/GIO APIs (e.g. `IconTheme`, `Image::from_icon_name`, etc.).
///
/// It registers the compiled `icons.gresource` bundle into the global GLib
/// resource system so GTK can resolve icon names from embedded assets.
///
/// # Important
///
/// - This does NOT load icons into GTK directly.
/// - It only makes them available via the GLib resource lookup system.
/// - The path used in GTK must match [`ICONS_RESOURCE_PATH`].
///
/// # Errors
///
/// Returns a [`gio::glib::Error`] if resource registration fails.
/// This typically happens when:
/// - the `.gresource` file was not embedded into the binary
/// - the build script did not generate `icons.gresource`
/// - the resource path inside the bundle is invalid
///
/// # Example
///
/// ```no_run
/// use nerd_gtk_icons::register_icons;
///
/// fn main() -> Result<(), gio::glib::Error> {
///     register_icons()?;
///
///     // After this, GTK can resolve icons from embedded resources
///     Ok(())
/// }
/// ```
pub fn register_icons() -> Result<(), gio::glib::Error> {
    gio::resources_register_include!("icons.gresource")
}

/// Root path inside the compiled GResource bundle where icons are stored.
///
/// This must match the `prefix` defined in `icons.gresource.xml`.
/// GTK uses this path when resolving icon names through `IconTheme`.
///
/// Example resource layout:
///
/// ```text
/// /com/nerd/icons/nf-linux-symbolic.svg
/// /com/nerd/icons/nf-github-symbolic.svg
/// ```
///
/// Used together with:
/// ```rust
/// theme.add_resource_path(ICONS_RESOURCE_PATH);
/// ```
pub const ICONS_RESOURCE_PATH: &str = "/com/nerd/icons";

/// Auto-generated Nerd Font icon name constants.
///
/// These constants are generated at compile time from `metadata.json`.
/// Each constant represents a GTK-compatible icon name (usually lowercase
/// with `-symbolic` suffix for symbolic rendering).
///
/// # Example
///
/// ```
/// use nerd_gtk_icons::icons::NF_LINUX_SYMBOLIC;
///
/// assert_eq!(NF_LINUX_SYMBOLIC, "nf-linux-symbolic");
/// ```
pub mod icons {
    include!(concat!(env!("OUT_DIR"), "/icons.rs"));
}

/// Reverse mapping from Unicode codepoints to icon names.
///
/// This module provides lookup from a Nerd Font glyph codepoint (`u32`)
/// to its corresponding GTK icon name.
///
/// Useful when converting font-based icons into GTK icon names dynamically.
///
/// # Example
///
/// ```
/// use nerd_gtk_icons::codepoint_map::ICONS;
///
/// let name = ICONS.get(&0xE68B);
/// assert_eq!(name, Some(&"nf-linux-symbolic"));
/// ```
pub mod codepoint_map {
    include!(concat!(env!("OUT_DIR"), "/codemap.rs"));
}
