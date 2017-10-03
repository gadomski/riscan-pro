use Result;
use nalgebra::Matrix4;
use std::path::{Path, PathBuf};

const PROJECT_RSP: &'static str = "project.rsp";

/// Returns the canonical rsp (xml) path for the provided path.
///
/// If the path already points to the xml file, the path is simply canonicalized. If the path is a
/// .RiSCAN directory, the xml path is appended. Otherwise, an `Error::InvalidProjectPath` is
/// returned.
///
/// # Examples
///
/// ```
/// let path = riscan_pro::rsp_path("data/project.RiSCAN").unwrap();
/// assert_eq!(std::fs::canonicalize("data/project.RiSCAN/project.rsp").unwrap(), path);
/// assert!(riscan_pro::rsp_path("data").is_err());
/// ```
pub fn rsp_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
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

/// A RiSCAN Pro project.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Project {
    pop: Matrix4<f64>,
}

impl Project {
    /// Reads a project from a path.
    ///
    /// This path can either be the `.RiSCAN` directory, or the underlying `project.rsp` file.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project1 = Project::from_path("data/project.RiSCAN").unwrap();
    /// let project2 = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
    /// assert_eq!(project1, project2);
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Project> {
        unimplemented!()
    }

    /// Returns this project's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let mut project = Project::new();
    /// let pop = project.pop();
    /// ```
    pub fn pop(&self) -> Matrix4<f64> {
        self.pop
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
