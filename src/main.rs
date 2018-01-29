#[macro_use]
extern crate clap;
extern crate riscan_pro;
extern crate serde_json;

use clap::App;
use riscan_pro::{Project, utils};
use std::fs::File;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let path = matches.value_of("PROJECT").unwrap();
    let project = Project::from_path(path).expect("Unable to create project");
    if let Some(matches) = matches.subcommand_matches("json") {
        let json = if matches.is_present("compact") {
            serde_json::to_string(&project).expect("Unable to serialize project")
        } else {
            serde_json::to_string_pretty(&project).expect("Unable to serialize project")
        };
        println!("{}", json);
    } else if let Some(matches) = matches.subcommand_matches("sop") {
        let path = matches.value_of("PATH").unwrap();
        for (name, scan_position) in &project.scan_positions {
            if matches.is_present("frozen") && !scan_position.is_frozen {
                continue;
            }
            let mut path = Path::new(path).to_path_buf();
            path.push(format!("{}.dat", name));
            let mut file = File::create(path).unwrap();
            utils::write_projective3(file, &scan_position.sop).unwrap();
        }
    } else if let Some(matches) = matches.subcommand_matches("pop") {
        let mut path = Path::new(matches.value_of("PATH").unwrap()).to_path_buf();
        path.push(project.name + ".dat");
        let mut file = File::create(path).unwrap();
        utils::write_projective3(file, &project.pop).unwrap();
    }
}
