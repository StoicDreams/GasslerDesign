use crate::prelude::*;

pub fn nav_content(contexts: &Contexts) -> Html {
    html! {
        <>
            <webui-flex justify="center" slot="header">
                <Image src="Logo.svg" title="Gassler Design Logo" />
            </webui-flex>
            <NavDisplay routes={get_nav_routing(contexts)} class="d-flex flex-column pa-1" />
        </>
    }
}

pub(crate) fn get_nav_routing(_contexts: &Contexts) -> Vec<NavRoute> {
    let nav_routes = vec![
        NavLinkInfo::link(
            "Home",
            "/",
            &FaIcon::duotone("house"),
            roles::PUBLIC,
            page_index,
        ),
        NavGroupInfo::link(
            "Classes",
            &FaIcon::duotone("file-code"),
            roles::INVALID,
            vec![NavLinkInfo::link(
                "Home",
                "/inspiring_interior_design",
                &FaIcon::duotone("house"),
                roles::PUBLIC,
                page_home,
            )],
        ),
        NavLinkInfo::link(
            "Gallery",
            "/gallery",
            &FaIcon::duotone("gallery-thumbnails"),
            roles::PUBLIC,
            page_gallery,
        ),
        NavLinkInfo::link(
            "About",
            "/about",
            &FaIcon::duotone("circle-info"),
            roles::PUBLIC,
            page_about,
        ),
        NavLinkInfo::link(
            "Consultation",
            "/consultation",
            &FaIcon::duotone("messages-question"),
            roles::PUBLIC,
            page_consultation,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            &FaIcon::duotone("handshake"),
            roles::PUBLIC,
            starter_page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            &FaIcon::duotone("shield-exclamation"),
            roles::PUBLIC,
            starter_page_privacy,
        ),
    ];
    nav_routes
}
