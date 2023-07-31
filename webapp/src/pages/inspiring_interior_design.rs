use crate::prelude::*;

pub(crate) fn page_home(_contexts: Contexts) -> Html {
    set_title("Inspiring Interior Design Rendering");
    html! {
        <>
            <Paper class="d-flex justify-center align-center" style={BG_CONTAINER}>
                <Paper style={BG_IMAGE} elevation={ELEVATION_STANDARD}>
                    <Paper style={BG_HCONTAINER} class="d-flex justify-center align-end theme-inherit">
                        <h2 style={TITLE_STYLES}>{"Inspired Design!"}</h2>
                    </Paper>
                </Paper>
            </Paper>

            <GDNext display="Gassler Design Gallery" url="/gallery" />
        </>
    }
}

const BG_CONTAINER: &str =
    "min-height:calc(var(--main-height) * 0.8);padding:calc(var(--main-width) * 0.03);";
const BG_IMAGE: &str = "width:90%;height:90%;margin:auto;border-radius:30px;background-image:url(/img/80.jpg);background-attachment:local;background-size:cover;background-position:bottom right;background-repeat:no-repeat;";
const BG_HCONTAINER: &str = "width: 100%;height:100%;";
const TITLE_STYLES: &str = "margin-left: calc(var(--main-width) * -0.05); color:white; font-size: 4em;position:sticky;left:0;text-shadow:-2px 2px 4px black;";
