mod mysvg;
mod svgtopng;
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
    create_svg(&args.in_file, &args.out_file);
    render_svg(args.out_file);
}
