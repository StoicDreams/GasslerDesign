use crate::prelude::*;

pub(crate) fn page_consultation(_contexts: &Contexts) -> Html {
    set_title("Consultation for interior design or rendering services");
    html! {
        <>
            <MarkdownContent href="/d/en-US/consultation.md" />
            <GDNext display="Inspiring Interior Design" url="/" />
        </>
    }
}
