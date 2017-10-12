extern crate clap;
extern crate las;
extern crate riscan_pro;
extern crate palette;
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
                .arg(Arg::with_name("IMAGE_DIR").index(2).required(true).help(
                    "Directory of images from which to pull color information",
                ))
                .arg(Arg::with_name("OUTFILE").index(3).required(true).help(
                    "Out las file",
                ))
                .arg(Arg::with_name("sync-to-pps").long("sync-to-pps").help(
                    "Only read points collected when the PPS from the GPS was synced",
                ))
                .arg(
                    Arg::with_name("rgb-domain")
                        .long("rgb-domain")
                        .help(
                            "The comma-seperated domain of temperatures over which to scale the rgb colorization",
                        )
                        .takes_value(true)
                        .default_value("-40,0"),
                )
                .arg(
                    Arg::with_name("rgb-range")
                        .long("rgb-range")
                        .help("The comma-seperated list of names to use for the output color range")
                        .takes_value(true)
                        .default_value("blue,red"),
                ),
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
    use las::{Header, Writer};
    use las::point::Color;
    use riscan_pro::{Colorizer, Point};
    use scanifc::point3d::Stream;
    use palette::{Gradient, Rgb, named};
    use std::fs;
    use std::io::{self, Write};

    print!("Opening point stream...");
    io::stdout().flush().unwrap();
    let stream = Stream::from_path(matches.value_of("INFILE").unwrap())
        .sync_to_pps(matches.is_present("sync-to-pps"))
        .open()
        .expect("Unable to open stream");
    println!("ok");

    println!("Creating colorizers...");
    let colorizers: Vec<_> = fs::read_dir(matches.value_of("IMAGE_DIR").unwrap())
        .expect("Could not read image directory")
        .filter_map(|result| {
            result.ok().and_then(|dir_entry| if dir_entry
                .path()
                .extension()
                .map(|s| s == "txt")
                .unwrap_or(false)
            {
                println!("  - {}", dir_entry.path().display());
                Some(Colorizer::from_path(dir_entry.path()).expect(
                    "Unable to create colorizer from image path",
                ))
            } else {
                None
            })
        })
        .collect();
    assert!(!colorizers.is_empty());
    // TODO validate that all colorizers have the same scan position
    println!("...done, with {} colorizers", colorizers.len());

    print!("Creating las writer...");
    io::stdout().flush().unwrap();
    let mut header = Header::default();
    header.point_format = 3.into();
    let mut writer = Writer::from_path(matches.value_of("OUTFILE").unwrap(), header)
        .expect("Could not create outfile");
    println!("done");

    let rgb_domain = matches
        .value_of("rgb-domain")
        .unwrap()
        .split(',')
        .map(|s| {
            s.parse::<f32>().expect(
                "Could not parse domain value as number",
            )
        })
        .collect::<Vec<_>>();
    assert!(
        rgb_domain.len() == 2,
        "rgb domain must be exactly two values"
    );
    let rgb_range = matches
        .value_of("rgb-range")
        .unwrap()
        .split(',')
        .map(|s| {
            Rgb::from_pixel(&named::from_str(s).expect(
                "Unable to convert name to color",
            ))
        })
        .collect::<Vec<_>>();
    assert!(rgb_range.len() == 2, "rgb range must be exactly two values");
    let gradient = Gradient::<Rgb>::with_domain(vec![
        (rgb_domain[0], rgb_range[0].into()),
        (rgb_domain[1], rgb_range[1].into()),
    ]);

    print!("Colorizing...");
    io::stdout().flush().unwrap();
    for point in stream {
        let point = point.unwrap();
        let socs = Point::socs(point.x, point.y, point.z);
        let colors = colorizers
            .iter()
            .filter_map(|colorizer| colorizer.colorize(&socs))
            .collect::<Vec<f64>>();
        if !colors.is_empty() {
            let color = colors.iter().sum::<f64>() / colors.len() as f64;
            let las_color = gradient.get(color as f32);
            let glcs = colorizers[0].socs_to_glcs(&socs);
            let point = las::Point {
                x: glcs.x,
                y: glcs.y,
                z: glcs.z,
                intensity: scale_reflectance(point.reflectance),
                color: Some(Color {
                    red: u16ify(las_color.red),
                    green: u16ify(las_color.green),
                    blue: u16ify(las_color.blue),
                }),
                gps_time: Some(color),
                ..Default::default()
            };
            writer.write(&point).unwrap();
        }
    }
    println!("done");

    fn scale_reflectance(reflectance: f32) -> u16 {
        use std::u16;
        let max = 20.;
        let min = -5.;
        ((reflectance - min) / (max - min) * u16::MAX as f32) as u16
    }

    fn u16ify(color: f32) -> u16 {
        use std::u16;
        assert!(color >= 0.);
        assert!(color <= 1.);
        (color * u16::MAX as f32) as u16
    }
}

#[cfg(not(feature = "scanifc"))]
fn colorize(_: &ArgMatches) {
    panic!("Colorization currently unsupported without scanifc (which requires Windows or Ubuntu)");
}
