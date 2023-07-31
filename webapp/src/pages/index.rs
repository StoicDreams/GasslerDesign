use crate::prelude::*;

pub(crate) fn page_index(_contexts: Contexts) -> Html {
    push_state("/inspiring_interior_design");
    pages::page_home(_contexts)
}
