extern crate docopt;
extern crate riscan_pro;
extern crate rustc_serialize;

use docopt::Docopt;
use riscan_pro::Project;

const USAGE: &'static str = "
RiSCAN PRO utilities.

Usage:
    riscan-pro socs-to-glcs <project> <scan-position> [<x> <y> <z>]
    riscan-pro scan-positions <project>
    riscan-pro (-h | --help)

Options:
    -h --help                       Show this screen.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_socs_to_glcs: bool,
    cmd_scan_positions: bool,
    arg_project: String,
    arg_scan_position: String,
    arg_x: Option<f64>,
    arg_y: Option<f64>,
    arg_z: Option<f64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let project = Project::from_path(args.arg_project).unwrap();
    if args.cmd_socs_to_glcs {
        let scan_position = project.scan_position(&args.arg_scan_position).unwrap();
        let glcs = scan_position.socs_to_glcs((args.arg_x.unwrap_or(0.),
                                               args.arg_y.unwrap_or(0.),
                                               args.arg_z.unwrap_or(0.)));
        println!("{:.2}, {:.2}, {:.2}", glcs.0, glcs.1, glcs.2);
    } else if args.cmd_scan_positions {
        for scan_position in project.scan_positions() {
            println!("{}", scan_position.name());
        }
    }
}
