mod camera_calibration;
mod image;
mod mount_calibration;
mod scan_position;
mod traits;

pub use project::camera_calibration::CameraCalibration;
pub use project::mount_calibration::MountCalibration;
pub use project::image::{Image, ImageData};

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

use xmltree::Element;

use {Error, Result};
use project::traits::GetDescendant;
use project::scan_position::ScanPosition;

#[derive(Debug, PartialEq)]
pub struct Project {
    scan_positions: HashMap<String, ScanPosition>,
}

impl Project {
    /// Reads a RiSCAN Pro project from a path.
    ///
    /// The path can either be the project directoy, which ends in `RiSCAN`, or the project XML
    /// file, `project.rsp`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// let project1 = Project::from_path("data/project.RiSCAN").unwrap();
    /// let project2 = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Project> {
        let mut path = path.as_ref().to_owned();
        if try!(fs::metadata(&path)).is_dir() {
            path.push("project.rsp");
        }
        let xml = try!(File::open(&path)
            .map_err(Error::from)
            .and_then(|file| Element::parse(file).map_err(Error::from)));
        let mount_calibrations = try!(xml.map_children("calibrations/mountcalibs", |child| {
            let mount_calibration = try!(MountCalibration::from_element(child));
            Ok((mount_calibration.name().to_string(), mount_calibration))
        }));
        let camera_calibrations = try!(xml.map_children("calibrations/camcalibs", |child| {
            let camera_calibration = try!(CameraCalibration::from_element(child));
            Ok((camera_calibration.name().to_string(), camera_calibration))
        }));
        let scan_positions = try!(xml.map_children("scanpositions", |child| {
            let scan_position = try!(ScanPosition::from_element(child,
                                                                path.parent().unwrap(),
                                                                &mount_calibrations,
                                                                &camera_calibrations));
            Ok((scan_position.name().to_string(), scan_position))
        }));
        Ok(Project { scan_positions: scan_positions })
    }

    /// Returns scan position with the provided name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = project.scan_position("SP01").unwrap();
    /// ```
    pub fn scan_position(&self, name: &str) -> Option<&ScanPosition> {
        self.scan_positions.get(name)
    }

    /// Returns the image of the provided name in the specified scan position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let image = project.image("SP01", "SP01 - Image001").unwrap();
    /// ```
    pub fn image(&self, scan_position: &str, image: &str) -> Option<&Image> {
        self.scan_position(scan_position).and_then(|scan_position| scan_position.image(image))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use point::{PRCS, Point};

    fn point() -> Point<PRCS, f64> {
        Point {
            crs: PRCS,
            x: -139.31727,
            y: -239.32973,
            z: -10.49305,
        }
    }

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

    #[test]
    fn project_scan_position() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        assert!(project.scan_position("SP01").is_some());
        assert!(project.scan_position("SP02").is_some());
        assert!(project.scan_position("SP03").is_none());
    }

    #[test]
    fn image_color() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let image = project.image("SP01", "SP01 - Image001").unwrap();
        assert_eq!(22.49, image.color(point()).unwrap().unwrap());
    }

    #[test]
    fn scan_position_color() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position("SP01").unwrap();
        assert_eq!(22.49, scan_position.color(point()).unwrap().unwrap());
    }
}
