extern crate docopt;
extern crate las;
extern crate riscan_pro;
extern crate rustc_serialize;

use std::path::Path;

use docopt::Docopt;
use las::{PointFormat, Reader, Writer};
use riscan_pro::{PRCS, Point, Project};

const USAGE: &'static str = "
RiSCAN PRO utilities.

Usage:
    riscan-pro colorize <infile> <project> <outfile> [--scan-position=<name>] [--allow-missing-images=<bool>]
    riscan-pro (-h | --help)

Options:
    -h --help                       Show this screen.
    --scan-position=<name>          Specify the scan position name. If not provided, we will try to deduce the scan position from the infile name.
    --allow-missing-images=<bool>   Allow colorization even if some images are missing from the scan position [default: false].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_colorize: bool,
    arg_infile: String,
    arg_project: String,
    arg_outfile: String,
    flag_scan_position: Option<String>,
    flag_allow_missing_images: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    if args.cmd_colorize {
        let project = Project::from_path(args.arg_project).unwrap();
        let scan_position = if let Some(name) = args.flag_scan_position {
            project.scan_position(&name).expect(&format!("Cound not find scan position: {}", name))
        } else {
            Path::new(&args.arg_infile)
                .file_stem()
                .and_then(|name| project.scan_position_with_scan(&name.to_string_lossy()))
                .expect(&format!("Could not deduce scan position from file: {}",
                                 args.arg_infile))
        };
        if !args.flag_allow_missing_images {
            let images = scan_position.missing_images();
            if !images.is_empty() {
                panic!("Scan position is missing images: {}",
                       images.into_iter().map(|i| i.name()).collect::<Vec<_>>().join(", "));
            }
        }
        let reader = Reader::from_path(args.arg_infile).unwrap();
        let mut writer = Writer::from_path(args.arg_outfile)
            .unwrap()
            .header(reader.header().clone())
            .point_format(PointFormat(1))
            .open()
            .unwrap();
        for mut las_point in reader {
            let point = Point {
                crs: PRCS,
                x: las_point.x,
                y: las_point.y,
                z: las_point.z,
            };
            if let Some(color) = scan_position.color(point).unwrap() {
                las_point.gps_time = Some(color);
                writer.write_point(&las_point).unwrap();
            }
        }
    }
}
