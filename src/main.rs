mod mysvg;
mod svgtopng;
use std::path::PathBuf;

use mysvg::create_svg;
use structopt::StructOpt;
use svgtopng::render_svg;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    in_file: std::path::PathBuf,
    #[structopt(parse(from_os_str))]
    out_file: std::path::PathBuf,
}
fn main() {
    let args = Cli::from_args();
    let mut out_path = std::env::current_dir().expect("Couldn't read current directory.");
    out_path.push("temp.svg");
    create_svg(&args.in_file, &out_path);
    render_svg(out_path, args.out_file);
}
