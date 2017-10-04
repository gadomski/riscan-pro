use {Error, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use sxd_document::Package;

const PROJECT_RSP: &'static str = "project.rsp";

fn rsp_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    use std::fs;
    use Error;

    let mut path = fs::canonicalize(path)?;
    if let Some(extension) = path.extension().map(|extension| {
                                                      extension.to_string_lossy().into_owned()
                                                  }) {
        match extension.as_str() {
            "RiSCAN" => {
                path.push(PROJECT_RSP);
                Ok(path)
            }
            "rsp" => Ok(path),
            _ => Err(Error::ProjectPath(path)),
        }
    } else {
        Err(Error::ProjectPath(path))
    }
}

#[derive(Debug)]
pub struct Rsp {
    package: Package,
}

impl Rsp {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Rsp> {
        use std::fs::File;
        use std::io::Read;

        let path = rsp_path(path)?;
        let mut file = File::open(path)?;
        let mut xml = String::new();
        file.read_to_string(&mut xml)?;
        xml.parse()
    }
}

impl FromStr for Rsp {
    type Err = Error;
    fn from_str(s: &str) -> Result<Rsp> {
        use sxd_document::parser;
        use regex::Regex;

        // Riegl uses ./project.dtd, which may not be xml legal.
        let s = s.replace("./project.dtd", "project.dtd");
        // And they put comments in the doctype. Why? Just why?
        let re =
            Regex::new(r"(?m)\[\s*<!-- PUT INTERNAL DOCUMENT TYPE DEFINITION SUBSET HERE -->\s*\]")
                .unwrap();
        let s = re.replace(&s, "");

        let package = parser::parse(&s)?;
        Ok(Rsp { package: package })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xml_path() -> PathBuf {
        use std::fs;
        fs::canonicalize("data/project.RiSCAN/project.rsp").unwrap()
    }

    #[test]
    fn rsp_path_from_rsp_path() {
        let path = rsp_path("data/project.RiSCAN/project.rsp").unwrap();
        assert_eq!(xml_path(), path);
    }

    #[test]
    fn rsp_path_from_riscan_path() {
        let path = rsp_path("data/project.RiSCAN").unwrap();
        assert_eq!(xml_path(), path);
    }

    #[test]
    fn rsp_path_err() {
        assert!(rsp_path("data").is_err());
        assert!(rsp_path("Cargo.toml").is_err());
    }
}
