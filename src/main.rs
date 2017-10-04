extern crate docopt;
extern crate riscan_pro;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use riscan_pro::Project;
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
        let project = Project::from_path(args.arg_project).unwrap();
        unimplemented!()
    }
}
