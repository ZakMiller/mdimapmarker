use std::path::PathBuf;

pub fn render_svg(path: PathBuf) {
    let path = std::path::Path::new(&path);
    let svg = nsvg::parse_file(path, nsvg::Units::Pixel, 96.0).unwrap();

    let image = svg.rasterize(1.0).unwrap();
    let (width, height) = image.dimensions();

    image::save_buffer(
        "image.png",
        &image.into_raw(),
        width,
        height,
        image::ColorType::RGBA(8),
    )
    .expect("Failed to save png.");
}
