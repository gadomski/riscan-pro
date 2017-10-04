use {Camera, Projective3, Result, ScanPosition};
use rsp::Rsp;
use std::collections::HashMap;
use std::path::Path;

/// A RiSCAN Pro project.
///
/// This project isn't a one-to-one mapping to Riegl's XML structure. We've chosen to cut cornerns
/// in order to easily support *our* use case. Specifically:
///
/// - Only one or zero camera calibrations are supported, not more than one.
#[derive(Debug, PartialEq)]
pub struct Project {
    camera: Option<Camera>,
    pop: Projective3,
    scan_positions: HashMap<String, ScanPosition>,
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
        let rsp = Rsp::from_path(path)?;
        Project::new(&rsp)
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

    /// Returns the scan position with the given name, or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// assert!(project.scan_position("SP01").is_some());
    /// assert!(project.scan_position("SP03").is_none());
    /// ```
    pub fn scan_position(&self, name: &str) -> Option<ScanPosition> {
        unimplemented!()
    }

    fn new(rsp: &Rsp) -> Result<Project> {
        Ok(Project {
               pop: rsp.projective3("/project/pop/matrix")?,
               camera: rsp.camera("/project/calibrations/camcalibs/camcalib_opencv")?,
               scan_positions: rsp.scan_positions("/project/scanpositions/scanposition")?,
           })
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
        let actual = project.pop();
        assert_relative_eq!(expected.matrix(), actual.matrix());
        let camera = Camera::from_path("data/camera.cam").unwrap();
        assert_eq!(camera, project.camera().unwrap());
        project.scan_position("SP01").unwrap();
        project.scan_position("SP02").unwrap();
        assert!(project.scan_position("SP03").is_none());
    }

    #[test]
    fn notaproject_rsp() {
        assert!(Project::from_path("data/notaproject.rsp").is_err());
    }
}
