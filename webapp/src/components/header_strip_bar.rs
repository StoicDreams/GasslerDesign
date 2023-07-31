use crate::prelude::*;

pub(crate) fn header_strip_bar(_contexts: Contexts) -> Html {
    html! {
        <>
            <span class="flex-break show-at-mobile" />
            <span class="flex-grow show-at-mobile" />
            <Paper>
                <Link class="btn theme-inherit pl-1 pr-1" icon="fa-brands fa-instagram"
                    title="Link to Gassler Design on Instagram"
                    href="https://www.instagram.com/gasslerdesign">
                </Link>
            </Paper>
        </>
    }
}
