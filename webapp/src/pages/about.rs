use crate::prelude::*;

pub(crate) fn page_about(_contexts: Contexts) -> Html {
    set_title("Consultation for Interior Design Rendering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/about.md" />
            <GDNext display="Consultation Services" url="/consultation" />
        </>
    }
}
