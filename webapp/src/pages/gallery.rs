use crate::prelude::*;

pub(crate) fn page_gallery(_contexts: &Contexts) -> Html {
    html!(<PageGallery />)
}

#[function_component(PageGallery)]
fn page_gallery_internal() -> Html {
    set_title("Inspiring Interior Design Rendering Gallery");
    let images = get_gallery_files();
    let copy = images.clone();
    let current_image = use_state(move || {
        let images = copy;
        images.first().unwrap().to_owned()
    });
    let image = current_image.path.to_string();
    let detail = current_image.detail.to_string();
    let style = "height: calc(0.8 * var(--main-height));";
    html! {
        <>
            <Paper class="d-flex justify-center align-center d-column pa-2" {style}>
                <Avatar {image} elevation={ELEVATION_STANDARD} style="height:100%;" />
            </Paper>
            <Paper class="d-flex flex-row justify-center pa-1 ma-1">
                {detail}
            </Paper>
            <Cards class="mb-5">
                {images.iter().map(move |image| {
                    let current_image = current_image.clone();
                    let image_path = image.path.to_string();
                    let image = image.clone();
                    html!(
                        <Card width={200}>
                            <a class="clickable" onclick={move |_| {
                                let current_image = current_image.clone();
                                let image = image.clone();
                                current_image.set(image);
                            }}>
                                <Avatar image={image_path} style="height:100px;" />
                            </a>
                        </Card>
                    )
                }).collect::<Html>()}
            </Cards>
            <GDNext display="About Gassler Design" url="/about" />
        </>
    }
}

fn get_gallery_files() -> Vec<ImageInfo> {
    vec![
        ImageInfo::new("/img/1.avif", "Living area concept"),
        ImageInfo::new("/img/6.avif", "Living area concept"),
        ImageInfo::new("/img/7.avif", "Living area concept"),
        ImageInfo::new("/img/9.avif", "Living area concept"),
        ImageInfo::new("/img/13.avif", "Living area concept"),
        ImageInfo::new("/img/14.avif", "Living area concept"),
        ImageInfo::new("/img/15.avif", "Living area concept"),
        ImageInfo::new("/img/16.avif", "Living area concept"),
        ImageInfo::new("/img/57.avif", "Dining table concept"),
        ImageInfo::new("/img/58.avif", "Dining area concept"),
        ImageInfo::new("/img/64.avif", "Table top concept"),
        ImageInfo::new("/img/70.avif", "Living area concept"),
        ImageInfo::new("/img/71.avif", "Entertainment Center concept"),
        ImageInfo::new("/img/73.avif", "Living area concept"),
        ImageInfo::new("/img/74.avif", "Living area concept"),
        ImageInfo::new("/img/76.avif", "Dining concept"),
        ImageInfo::new("/img/77.avif", "Kitchen concept"),
        ImageInfo::new("/img/80.avif", "Living area concept"),
        ImageInfo::new("/img/bedroom.avif", "Bedroom concept"),
    ]
}

#[derive(Clone, PartialEq, Debug)]
struct ImageInfo {
    pub path: String,
    pub detail: String,
}

impl ImageInfo {
    pub fn new(path: &str, detail: &str) -> Self {
        Self {
            path: path.to_string(),
            detail: detail.to_string(),
        }
    }
}
