use std::fs;
use std::path::Path;

use {Error, Result};

#[derive(Debug, PartialEq)]
pub struct Project;

impl Project {
    /// Reads a RiSCAN Pro project from a path.
    ///
    /// The path can either be the project directoy, which ends in `RiSCAN`, or the project XML
    /// file, `project.rsp`.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project1 = Project::from_path("data/project.RiSCAN").unwrap();
    /// let project2 = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Project> {
        fs::metadata(path).map(|_| Project).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_from_path_ok() {
        let project1 = Project::from_path("data/project.RiSCAN").unwrap();
        let project2 = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
        assert_eq!(project1, project2);
    }

    #[test]
    fn project_from_path_not_ok() {
        assert!(Project::from_path("notaproject").is_err());
    }
}
