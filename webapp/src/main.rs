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
    .set_nav_routing(NavRoutingCallback::new(nav_menu::get_nav_routing))
    .set_navigation(nav_menu::nav_content)
    .set_header(app_header)
    .set_copyright_start(2021)
    .build()
}

fn app_header(contexts: &Contexts) -> Html {
    let app_config = contexts.clone().config;
    let mut show_discord = false;
    html! {
        <>
            <h1>{ app_config.app_name.clone() }</h1>
            <h2 class="flex-grow" data-subsribe="page-title"></h2>
            <span class="flex-break show-at-mobile" />
            <span class="flex-grow show-at-mobile" />
            {header_strip_bar(contexts)}
            <webui-feedback title="Provide us your feedback!" data-post="https://api.myfi.ws/feedback/new" data-json-name="message"></webui-feedback>
            <webui-alerts title="View My Alerts" data-title="My Alerts" data-toggleclass=".shared|open"></webui-alerts>
            {myfi_info_panel(contexts)}
        </>
    }
}
