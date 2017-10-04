extern crate docopt;
extern crate irb;
extern crate las;
extern crate riscan_pro;
extern crate scanifc;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use irb::text::File;
use riscan_pro::Project;
use scanifc::point3d::Stream;
use std::path::PathBuf;

const USAGE: &'static str = "
Query and work with RiSCAN Pro projects.

Usage:
    riscan-pro colorize <project> <rxp> <image>

Options:
    -h --help           Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_colorize: bool,
    arg_project: PathBuf,
    arg_rxp: PathBuf,
    arg_image: PathBuf,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.cmd_colorize {
        colorize(&args);
    }
}

fn colorize(args: &Args) {
    let project = Project::from_path(&args.arg_project).expect("Could not open project");
    let stream = Stream::from_path(&args.arg_rxp).open().expect(
        "Could not open stream",
    );
    let image = File::open(&args.arg_image)
        .expect("Could not open image")
        .into_image()
        .expect("Could not read image");
    for point in stream {}
    unimplemented!()
}
