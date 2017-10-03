use {Camera, Error, Projective3, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;

const PROJECT_RSP: &'static str = "project.rsp";

/// Returns the canonical rsp (xml) path for the provided path.
///
/// If the path already points to the xml file, the path is simply canonicalized. If the path is a
/// .RiSCAN directory, the xml path is appended. Otherwise, an `Error::ProjectPath` is returned.
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
///
/// This project isn't a one-to-one mapping to Riegl's XML structure. We've chosen to cut cornerns
/// in order to easily support *our* use case. Specifically:
///
/// - Only one or zero camera calibrations are supported, not more than one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Project {
    camera: Option<Camera>,
    pop: Projective3,
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
        use std::fs::File;
        use std::io::Read;

        let path = rsp_path(path)?;
        let mut file = File::open(path)?;
        let mut xml = String::new();
        file.read_to_string(&mut xml)?;
        xml.parse()
    }

    /// Returns this project's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let pop = project.pop();
    /// ```
    pub fn pop(&self) -> Projective3 {
        self.pop
    }

    /// Returns this project's camera calibration, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let camera = project.camera().unwrap();
    /// ```
    pub fn camera(&self) -> Option<Camera> {
        self.camera
    }
}

impl FromStr for Project {
    type Err = Error;

    fn from_str(s: &str) -> Result<Project> {
        use sxd_document::parser;
        use utils;

        // Riegl uses ./project.dtd, which may not be xml legal.
        let s = s.replace("./project.dtd", "project.dtd");
        let package = parser::parse(&s)?;
        let document = package.as_document();

        let pop = utils::projective_from_str(&xpath!(&document, "/project/pop/matrix").string())?;
        let camera = Camera::from_document(&document)?;
        Ok(Project {
               camera: camera,
               pop: pop,
           })
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

    #[test]
    fn project() {
        use Camera;
        use nalgebra::Matrix4;

        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let expected = Projective3::from_matrix_unchecked(Matrix4::new(0.99566497679815923,
                                                                       0.046111730526226816,
                                                                       -0.080777238659154112,
                                                                       -515632.66332186362,
                                                                       -0.093012117369304602,
                                                                       0.49361133154539053,
                                                                       -0.86469451217899213,
                                                                       -5519682.7927730317,
                                                                       0.,
                                                                       0.86845930340912512,
                                                                       0.49576046466225683,
                                                                       3143447.4201939853,
                                                                       0.,
                                                                       0.,
                                                                       0.,
                                                                       1.));
        let actual = project.pop();
        assert_relative_eq!(expected.matrix(), actual.matrix());
        let camera = Camera::from_path("data/camera.cam").unwrap();
        assert_eq!(camera, project.camera().unwrap());
    }

    #[test]
    fn empty_rsp() {
        assert!(Project::from_path("data/empty.rsp").is_err());
    }

    #[test]
    fn notaproject_rsp() {
        assert!(Project::from_path("data/notaproject.rsp").is_err());
    }
}
