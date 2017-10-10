extern crate clap;
extern crate riscan_pro;
#[cfg(feature = "scanifc")]
extern crate scanifc;
extern crate serde_json;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use riscan_pro::{Colorizer, Point, Project};

fn main() {
    let matches = App::new("RiSCAN Pro")
        .author("Pete Gadomski <pete@gadom.ski>")
        .about("Query RiSCAN Pro projects")
        .subcommand(
            SubCommand::with_name("info")
                .about("Show info about the project")
                .arg(Arg::with_name("PROJECT").index(1).required(true).help(
                    "path to the project",
                )),
        )
        .subcommand(
            SubCommand::with_name("colorize")
                .about("Colorize a point cloud")
                .arg(Arg::with_name("INFILE").index(1).required(true).help(
                    "Input point cloud",
                ))
                .arg(Arg::with_name("IMAGE").index(2).required(true).help(
                    "Image from which to draw color information",
                ))
                .arg(Arg::with_name("sync-to-pps").long("sync-to-pps").help(
                    "Only read points collected when the PPS from the GPS was synced",
                )),
        )
        .subcommand(
            SubCommand::with_name("pixel")
                .setting(AppSettings::AllowLeadingHyphen)
                .about("Convert SOCS coordinate to image coordinates")
                .arg(Arg::with_name("IMAGE").index(1).required(true).help(
                    "the image",
                ))
                .arg(Arg::with_name("X").index(2).required(true).help(
                    "x coordinate",
                ))
                .arg(Arg::with_name("Y").index(3).required(true).help(
                    "y coordinate",
                ))
                .arg(Arg::with_name("Z").index(4).required(true).help(
                    "z coordinate",
                )),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("info") {
        let path = matches.value_of("PROJECT").unwrap();
        let project = Project::from_path(path).expect("Unable to create project");
        println!(
            "{}",
            serde_json::to_string_pretty(&project).expect("Unable to serialize project")
        );
    }

    if let Some(matches) = matches.subcommand_matches("pixel") {
        let colorizer = Colorizer::from_path(matches.value_of("IMAGE").unwrap())
            .expect("Could not create colorizer for image");
        let socs = Point::socs(
            matches.value_of("X").unwrap().parse::<f64>().unwrap(),
            matches.value_of("Y").unwrap().parse::<f64>().unwrap(),
            matches.value_of("Z").unwrap().parse::<f64>().unwrap(),
        );
        if let Some((u, v)) = colorizer.pixel(&socs) {
            println!("u={}, v={}", u, v);
        } else {
            println!("None");
        }
    }

    if let Some(matches) = matches.subcommand_matches("colorize") {
        colorize(&matches);
    }
}

#[cfg(feature = "scanifc")]
fn colorize(matches: &ArgMatches) {
    use scanifc::point3d::Stream;
    use riscan_pro::{Colorizer, Point};

    let stream = Stream::from_path(matches.value_of("INFILE").unwrap())
        .sync_to_pps(matches.is_present("sync-to-pps"))
        .open()
        .expect("Unable to open stream");
    let colorizer = Colorizer::from_path(matches.value_of("IMAGE").unwrap()).unwrap();
    for point in stream {
        let point = point.unwrap();
        let socs = Point::socs(point.x, point.y, point.z);
        if let Some(color) = colorizer.colorize(&socs) {
            println!("{},{},{},{}", socs.x, socs.y, socs.z, color);
        }
    }
}

#[cfg(not(feature = "scanifc"))]
fn colorize(_: &ArgMatches) {
    panic!("Colorization currently unsupported without scanifc (which requires Windows or Ubuntu)");
}
