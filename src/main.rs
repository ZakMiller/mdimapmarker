mod mysvg;
mod svgtopng;
use mysvg::create_svg;
use svgtopng::render_svg;
fn main() {
    let in_file = String::from("images/account-group.svg");
    let out_file = String::from("image.svg");
    create_svg(&in_file, &out_file);
    render_svg(out_file);
}
