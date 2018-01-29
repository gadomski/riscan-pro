#[macro_use]
extern crate clap;
extern crate riscan_pro;
extern crate serde_json;

use clap::App;
use riscan_pro::Project;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    if let Some(matches) = matches.subcommand_matches("json") {
        let path = matches.value_of("PROJECT").unwrap();
        let project = Project::from_path(path).expect("Unable to create project");
        let json = if matches.is_present("compact") {
            serde_json::to_string(&project).expect("Unable to serialize project")
        } else {
            serde_json::to_string_pretty(&project).expect("Unable to serialize project")
        };
        println!("{}", json);
    }
}
