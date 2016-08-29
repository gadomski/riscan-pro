mod camera_calibration;
mod image;
mod mount_calibration;
mod scan;
mod scan_position;
mod traits;

pub use project::camera_calibration::CameraCalibration;
pub use project::mount_calibration::MountCalibration;
pub use project::image::{Image, ImageData};
pub use project::scan::Scan;
pub use project::scan_position::ScanPosition;

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use nalgebra::Matrix4;
use xmltree::Element;

use {Error, Result};
use project::traits::GetDescendant;

/// A RiSCAN Pro project.
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

    /// Finds the scan position that contains the named scan.
    ///
    /// Since scans are named by their timestamp, it is practically impossible to have two scans
    /// with the same name in a project.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = project.scan_position_with_scan("151120_150227").unwrap();
    /// ```
    pub fn scan_position_with_scan(&self, name: &str) -> Option<&ScanPosition> {
        self.scan_positions.values().filter(|&s| s.contains_scan(name)).next()
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

impl ScanPosition {
    fn from_element<P>(element: &Element,
                       project_path: P,
                       mount_calibrations: &HashMap<String, MountCalibration>,
                       camera_calibrations: &HashMap<String, CameraCalibration>)
                       -> Result<ScanPosition>
        where P: AsRef<Path>
    {
        let name = try!(element.get_text("name"));
        let sop = try!(element.get_matrix4("sop/matrix"));
        let scans = try!(element.map_children("singlescans", |child| {
            let scan = try!(Scan::from_element(child));
            Ok((scan.name().to_string(), scan))
        }));
        let images = try!(element.map_children("scanposimages", |child| {
            let ref mount_calibration = try!(child.get_noderef("mountcalib_ref")
                .and_then(|name| {
                    mount_calibrations.get(name)
                        .ok_or(Error::MissingElement(format!("mount_calibration[name={}]", name)))
                }));
            let ref camera_calibration = try!(child.get_noderef("camcalib_ref")
                .and_then(|name| {
                    camera_calibrations.get(name)
                        .ok_or(Error::MissingElement(format!("camera_calibration[name={}]", name)))
                }));
            let image = try!(Image::from_element(child,
                                                 &project_path,
                                                 (*mount_calibration).clone(),
                                                 (*camera_calibration).clone(),
                                                 name,
                                                 sop));
            Ok((image.name().to_string(), image))
        }));
        Ok(ScanPosition::new(name, images, scans))
    }
}

impl Image {
    fn from_element<P>(element: &Element,
                       project_path: P,
                       mount_calibration: MountCalibration,
                       camera_calibration: CameraCalibration,
                       scan_position_name: &str,
                       sop: Matrix4<f64>)
                       -> Result<Image>
        where P: AsRef<Path>
    {
        let mut path = PathBuf::from(format!("{}/SCANS/{}/SCANPOSIMAGES",
                                             project_path.as_ref().to_string_lossy(),
                                             scan_position_name));
        path.push(try!(element.get_text("file")));
        let image_data = try!(image::read_image_data(path));
        Ok(Image::new(scan_position_name,
                      sop,
                      try!(element.get_text("name")),
                      mount_calibration,
                      camera_calibration,
                      try!(element.get_matrix4("cop/matrix")),
                      image_data))
    }
}

impl CameraCalibration {
    fn from_element(element: &Element) -> Result<CameraCalibration> {
        match element.name.as_str() {
            "camcalib_opencv" => {
                Ok(CameraCalibration::OpenCv {
                    name: try!(element.get_text("name")).to_string(),
                    cameramodel: try!(element.get_text("cameramodel")).to_string(),
                    version: try!(element.parse("version")),
                    angle_extents: camera_calibration::opencv::AngleExtents {
                        tan_max_horz: try!(element.parse("angle_extents/tan_max_horz")),
                        tan_max_vert: try!(element.parse("angle_extents/tan_max_vert")),
                        tan_min_horz: try!(element.parse("angle_extents/tan_min_horz")),
                        tan_min_vert: try!(element.parse("angle_extents/tan_min_vert")),
                    },
                    internal_opencv: camera_calibration::opencv::Internal {
                        cx: try!(element.parse("internal_opencv/cx")),
                        cy: try!(element.parse("internal_opencv/cy")),
                        fx: try!(element.parse("internal_opencv/fx")),
                        fy: try!(element.parse("internal_opencv/fy")),
                        k1: try!(element.parse("internal_opencv/k1")),
                        k2: try!(element.parse("internal_opencv/k2")),
                        k3: try!(element.parse("internal_opencv/k3")),
                        k4: try!(element.parse("internal_opencv/k4")),
                        p1: try!(element.parse("internal_opencv/p1")),
                        p2: try!(element.parse("internal_opencv/p2")),
                    },
                    intrinsic_opencv: camera_calibration::opencv::Intrinsic {
                        dx: try!(element.parse("intrinsic_opencv/dx")),
                        dy: try!(element.parse("intrinsic_opencv/dy")),
                        nx: try!(element.parse("intrinsic_opencv/nx")),
                        ny: try!(element.parse("intrinsic_opencv/ny")),
                    },
                })
            }
            _ => Err(Error::UnsupportedCameraCalibration(element.name.to_string())),
        }
    }
}

impl MountCalibration {
    fn from_element(element: &Element) -> Result<MountCalibration> {
        Ok(MountCalibration::new(try!(element.get_text("name")),
                                 try!(element.get_matrix4("matrix"))))
    }
}

impl Scan {
    fn from_element(element: &Element) -> Result<Scan> {
        Ok(Scan::new(try!(element.get_text("name"))))
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

    #[test]
    fn project_scan_position_with_scan() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position_with_scan("151120_150227").unwrap();
        assert_eq!(project.scan_position("SP01").unwrap(), scan_position);
        let scan_position = project.scan_position_with_scan("151120_155528").unwrap();
        assert_eq!(project.scan_position("SP02").unwrap(), scan_position);
        assert!(project.scan_position_with_scan("151120_155529").is_none());
    }
}
