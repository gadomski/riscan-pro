#[macro_use]
extern crate clap;
extern crate riscan_pro;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use riscan_pro::Project;

fn main() {
    let matches = App::new("RiSCAN Pro")
        .author("Pete Gadomski <pete@gadom.ski>")
        .about("Query RiSCAN Pro projects")
        .subcommand(
            SubCommand::with_name("info")
                .about("Show info about the project")
                .version(crate_version!())
                .arg(Arg::with_name("PROJECT").index(1).required(true).help(
                    "path to the project",
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
}
