use {Camera, Error, Result, ScanPosition};
use element::Extension;
use nalgebra::Projective3;
use std::path::Path;
use xmltree::Element;

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
        use utils;

        let path = utils::rsp_path(path)?;
        let mut file = File::open(path)?;
        let element = Element::parse(&mut file)?;
        if element.name != "project" {
            Err(Error::InvalidRspRoot(element.name))
        } else {
            Ok(Project { root: element })
        }
    }

    /// Returns this project's POP matrix.
    ///
    /// This is the project's own position. When combined with the scanner's own position, can take
    /// a point in the scanner's own coordinate system and convert it to a global coordinate
    /// system.
    ///
    /// # Examples
    ///
    /// ```
    /// #[macro_use]
    /// extern crate approx;
    /// # extern crate riscan_pro;
    /// # fn main() {
    /// use riscan_pro::{Project, point};
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let pop = project.pop().unwrap();
    /// let prcs = point::prcs(1., 2., 3.);
    /// let glcs = prcs.to_glcs(pop);
    /// let prcs2 = glcs.to_prcs(pop);
    /// assert_relative_eq!(prcs.as_point3(), prcs2.as_point3(), epsilon = 1e-7);
    /// # }
    /// ```
    pub fn pop(&self) -> Result<Projective3<f64>> {
        self.root.xpath("pop/matrix").and_then(|e| e.convert())
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
        let camcalibs = self.root.xpath("calibrations/camcalibs")?;
        if camcalibs.children.is_empty() {
            Ok(None)
        } else if camcalibs.children.len() > 1 {
            Err(Error::MultipleCameras)
        } else {
            camcalibs.children[0].convert().map(|camera| Some(camera))
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
        let scanpositions = self.root.xpath("scanpositions")?;
        scanpositions.children
            .iter()
            .find(|child| {
                      child.xpath("name")
                          .and_then(|e| e.as_str())
                          .map(|s| s == name)
                          .unwrap_or(false)
                  })
            .map(|e| e.convert().map(|s| Some(s)))
            .unwrap_or(Ok(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
