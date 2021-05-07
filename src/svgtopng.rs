use std::path::PathBuf;

pub fn render_svg(in_file: PathBuf, out_file: PathBuf) {
    let path = std::path::Path::new(&in_file);
    let svg = nsvg::parse_file(path, nsvg::Units::Pixel, 96.0).unwrap();

    let image = svg.rasterize(1.0).unwrap();
    let (width, height) = image.dimensions();

    image::save_buffer(
        out_file,
        &image.into_raw(),
        width,
        height,
        image::ColorType::RGBA(8),
    )
    .expect("Failed to save png.");
}
