use {Camera, Error, Projective3, Result, ScanPosition};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use xmltree::Element;

const PROJECT_RSP: &'static str = "project.rsp";

/// A RiSCAN Pro project.
///
/// This project isn't a one-to-one mapping to Riegl's XML structure. We've chosen to cut cornerns
/// in order to easily support *our* use case. Specifically:
///
/// - Only one or zero camera calibrations are supported, not more than one.
#[derive(Debug, PartialEq)]
pub struct Project {
    root: Element,
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

        let path = rsp_path(path)?;
        let mut file = File::open(path)?;
        let element = Element::parse(&mut file)?;
        if element.name != "project" {
            Err(Error::InvalidRspRoot(element.name))
        } else {
            Ok(Project { root: element })
        }
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
    pub fn pop(&self) -> Result<Projective3> {
        projective3_from_xpath(&self.root, "pop/matrix")
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
    pub fn camera(&self) -> Result<Option<Camera>> {
        let camcalibs = xpath(&self.root, "calibrations/camcalibs")?;
        if camcalibs.children.is_empty() {
            Ok(None)
        } else if camcalibs.children.len() > 1 {
            Err(Error::MultipleCameras)
        } else {
            let ref camera = camcalibs.children[0];
            Ok(Some(Camera {
                        fx: parse_from_xpath(camera, "internal_opencv/fx")?,
                        fy: parse_from_xpath(camera, "internal_opencv/fy")?,
                        cx: parse_from_xpath(camera, "internal_opencv/cx")?,
                        cy: parse_from_xpath(camera, "internal_opencv/cy")?,
                        k1: parse_from_xpath(camera, "internal_opencv/k1")?,
                        k2: parse_from_xpath(camera, "internal_opencv/k2")?,
                        k3: parse_from_xpath(camera, "internal_opencv/k3")?,
                        k4: parse_from_xpath(camera, "internal_opencv/k4")?,
                        p1: parse_from_xpath(camera, "internal_opencv/p1")?,
                        p2: parse_from_xpath(camera, "internal_opencv/p2")?,
                        nx: parse_from_xpath(camera, "intrinsic_opencv/nx")?,
                        ny: parse_from_xpath(camera, "intrinsic_opencv/ny")?,
                        dx: parse_from_xpath(camera, "intrinsic_opencv/dx")?,
                        dy: parse_from_xpath(camera, "intrinsic_opencv/dy")?,
                    }))
        }
    }

    /// Returns the scan position with the given name, or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// assert!(project.scan_position("SP01").unwrap().is_some());
    /// assert!(project.scan_position("SP03").unwrap().is_none());
    /// ```
    pub fn scan_position(&self, name: &str) -> Result<Option<ScanPosition>> {
        let scanpositions = xpath(&self.root, "scanpositions")?;
        Ok(scanpositions.children
               .iter()
               .find(|child| str_from_xpath(child, "name").map(|s| s == name).unwrap_or(false))
               .map(|_| ScanPosition {}))
    }
}

fn rsp_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
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

fn parse_from_xpath<T>(element: &Element, xpath: &str) -> Result<T>
    where T: FromStr,
          Error: From<<T as FromStr>::Err>
{
    str_from_xpath(element, xpath).and_then(|s| s.parse().map_err(Error::from))
}

fn projective3_from_xpath(element: &Element, xpath: &str) -> Result<Projective3> {
    use utils;
    let s = str_from_xpath(element, xpath)?;
    utils::projective_from_str(s)
}

fn str_from_xpath<'a>(element: &'a Element, s: &str) -> Result<&'a str> {
    let element = xpath(element, s)?;
    if let Some(s) = element.text.as_ref() {
        Ok(s)
    } else {
        Err(Error::NoText(element.clone()))
    }
}

fn xpath<'a>(mut element: &'a Element, xpath: &str) -> Result<&'a Element> {
    for name in xpath.split('/') {
        if let Some(child) = element.get_child(name) {
            element = child;
        } else {
            return Err(Error::MissingChild(xpath.to_string()));
        }
    }
    Ok(element)
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
        let actual = project.pop().unwrap();
        assert_relative_eq!(expected.matrix(), actual.matrix());
        let camera = Camera::from_path("data/camera.cam").unwrap();
        assert_eq!(camera, project.camera().unwrap().unwrap());
        project.scan_position("SP01").unwrap().unwrap();
        project.scan_position("SP02").unwrap().unwrap();
        assert!(project.scan_position("SP03").unwrap().is_none());
    }

    #[test]
    fn notaproject_rsp() {
        assert!(Project::from_path("data/notaproject.rsp").is_err());
    }

    #[test]
    fn empty_rsp() {
        assert!(Project::from_path("data/empty.rsp").is_err());
    }

    #[test]
    fn two_cameras() {
        let project = Project::from_path("data/two-cameras.rsp").unwrap();
        assert!(project.camera().is_err());
    }

    #[test]
    fn extra_crap_in_doctype() {
        Project::from_path("data/extra-crap-in-doctype.rsp").unwrap();
    }
}
