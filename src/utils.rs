use Result;
use nalgebra::Projective3;
use std::path::{Path, PathBuf};

const PROJECT_RSP: &'static str = "project.rsp";

pub fn rsp_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    use Error;
    use std::fs;

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

pub fn projective_from_str(s: &str) -> Result<Projective3<f64>> {
    use Error;
    use nalgebra::{self, Matrix4};
    s.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()
        .and_then(|v| {
            if v.len() != 16 {
                return Err(Error::ParseMatrix4(s.to_string()));
            }
            let matrix = Matrix4::from_iterator(v).transpose();
            nalgebra::try_convert(matrix).ok_or(Error::Inverse(matrix))
        })
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

    #[test]
    fn projective_from_str_error() {
        assert!(projective_from_str("").is_err());
    }
}
