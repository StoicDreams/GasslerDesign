#![allow(unused)] // TODO: Remove me when needing to check for dead code / unused methods/variables.
mod components;
mod nav_menu;
mod pages;
mod prelude;

use prelude::*;

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    AppConfig::builder(
        "Gassler Design".to_string(),
        "Gassler Design".to_string(),
        "https://www.gassler.design".to_owned(),
        "Gassler.Design".to_owned(),
    )
    .set_header_logo_src("Logo.svg".to_owned())
    .set_header_strip_bar(header_strip_bar)
    .set_nav_routing(nav_menu::get_nav_routing())
    .set_drawer_toggle_header_middle(myfi_feedback_button_info())
    .set_drawer_toggle_header_left(nav_menu::nav_menu_info())
    .set_copyright_start(2021)
    .build()
}
