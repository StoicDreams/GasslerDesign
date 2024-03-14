use crate::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct GDNextProps {
    pub display: Option<String>,
    pub url: String,
}

#[function_component(GDNext)]
pub(crate) fn gdnext(props: &GDNextProps) -> Html {
    let contexts = use_context::<Contexts>().expect("Contexts not found");
    let app_config = contexts.config.clone();
    let display = match props.display.to_owned() {
        Some(display) => display,
        None => match app_config.get_nav_from_path(&props.url, &contexts) {
            Some(nav) => nav.name,
            None => "Not Found".to_string(),
        },
    };
    let href = props.url.to_owned();
    html!(
        <>
            <Paper class="d-flex flex-column align-center justify-center mt-a">
                <Link class="f5" href="https://www.instagram.com/gasslerdesign">
                    <Avatar class="d-inline mr-3" icon="fa-brands fa-instagram" />
                    <span>{"Visit our Instagram"}</span>
                </Link>
            </Paper>
            <Paper class="d-flex flex-column align-center justify-center mt-10">
                <Link class="btn theme-info" {href}>
                    <span>{format!("Continue to {}", display)}</span>
                    <Avatar class="d-inline ml-2" icon="fa-duotone fa-right" />
                </Link>
            </Paper>
        </>
    )
}
