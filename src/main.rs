mod mysvg;
mod svgtopng;
use mysvg::create_svg;
use svgtopng::render_svg;
fn main() {
    let out_file_name = String::from("image.svg");
    create_svg(&out_file_name);
    render_svg(out_file_name);
}
