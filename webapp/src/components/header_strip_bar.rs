use crate::prelude::*;

pub(crate) fn header_strip_bar(_contexts: &Contexts) -> Html {
    html! {
        <>
            <Link class="btn theme-inherit pl-1 pr-1" icon={FaIcon::brands("instagram")}
                title="Link to Gassler Design on Instagram"
                href="https://www.instagram.com/gasslerdesign">
            </Link>
        </>
    }
}
