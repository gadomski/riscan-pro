extern crate docopt;
extern crate nalgebra;
extern crate riscan_pro;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use riscan_pro::{Project, ScanPosition};

const USAGE: &'static str = "
RiSCAN PRO utilities.

Usage:
    riscan-pro export-filters <project> [<scan-position>]
    riscan-pro socs-to-glcs <project> [<scan-position>] [<x> <y> <z>]
    riscan-pro scan-positions <project>
    riscan-pro tpl-glcs <project>
    riscan-pro (-h | --help)

Options:
    -h --help                       Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_export_filters: bool,
    cmd_socs_to_glcs: bool,
    cmd_scan_positions: bool,
    cmd_tpl_glcs: bool,
    arg_project: String,
    arg_scan_position: Option<String>,
    arg_x: Option<f64>,
    arg_y: Option<f64>,
    arg_z: Option<f64>,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.deserialize()).unwrap_or_else(|e| e.exit());
    if args.cmd_socs_to_glcs {
        let scan_position = if let Some(scan_position) = args.arg_scan_position {
            let project = Project::from_path(args.arg_project).unwrap();
            project.scan_position(&scan_position).unwrap().clone()
        } else {
            ScanPosition::from_path(args.arg_project).unwrap()
        };
        let glcs = scan_position.socs_to_glcs((args.arg_x.unwrap_or(0.),
                                               args.arg_y.unwrap_or(0.),
                                               args.arg_z.unwrap_or(0.)));
        println!("{:.2},{:.2},{:.2}", glcs.0, glcs.1, glcs.2);
    } else if args.cmd_scan_positions {
        let project = Project::from_path(args.arg_project).unwrap();
        for scan_position in project.scan_positions() {
            let glcs = scan_position.socs_to_glcs((0., 0., 0.));
            println!("{},{:.2},{:.2}", scan_position.name(), glcs.0, glcs.1);
        }
    } else if args.cmd_tpl_glcs {
        let project = Project::from_path(args.arg_project).unwrap();
        for tiepoint in project.tpl_glcs() {
            println!("{},{:.2},{:.2}", tiepoint.name, tiepoint.x, tiepoint.y);
        }
    } else if args.cmd_export_filters {
        unimplemented!()
    }
}
