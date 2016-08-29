mod camera_calibration;
mod traits;

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

use nalgebra::{Inverse, Matrix3, Matrix4, Vector3, Vector4};
use xmltree::Element;

use {Error, Result};
use point::{PRCS, Point};
use project::traits::GetDescendant;
use project::camera_calibration::CameraCalibration;

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
        let xml = try!(File::open(path)
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
            let scan_position =
                try!(ScanPosition::from_element(child, &mount_calibrations, &camera_calibrations));
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

/// A fixed postion where one or more scans were taken, along with optional pictures and other data.
#[derive(Debug, PartialEq)]
pub struct ScanPosition {
    name: String,
    images: HashMap<String, Image>,
}

impl ScanPosition {
    fn from_element(element: &Element,
                    mount_calibrations: &HashMap<String, MountCalibration>,
                    camera_calibrations: &HashMap<String, CameraCalibration>)
                    -> Result<ScanPosition> {
        let sop = try!(element.get_matrix4("sop/matrix"));
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
                                                 (*mount_calibration).clone(),
                                                 (*camera_calibration).clone(),
                                                 sop));
            Ok((image.name().to_string(), image))
        }));
        Ok(ScanPosition {
            name: try!(element.get_text("name")).to_string(),
            images: images,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self, point: Point<PRCS, f64>) -> Result<Option<f64>> {
        for image in self.images() {
            let color = try!(image.color(point));
            if color.is_some() {
                return Ok(color);
            }
        }
        Ok(None)
    }

    pub fn images(&self) -> &Vec<Image> {
        unimplemented!()
    }

    pub fn image(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}

#[derive(Debug, PartialEq)]
pub struct Image {
    cop: Matrix4<f64>,
    camera_calibration: CameraCalibration,
    mount_calibration: MountCalibration,
    name: String,
    sop: Matrix4<f64>,
}

impl Image {
    fn from_element(element: &Element,
                    mount_calibration: MountCalibration,
                    camera_calibration: CameraCalibration,
                    sop: Matrix4<f64>)
                    -> Result<Image> {
        Ok(Image {
            camera_calibration: camera_calibration,
            cop: try!(element.get_matrix4("cop/matrix")),
            mount_calibration: mount_calibration,
            name: try!(element.get_text("name")).to_string(),
            sop: sop,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self, point: Point<PRCS, f64>) -> Result<Option<f64>> {
        let (u, v) = try!(self.project(point));
        unimplemented!()
    }

    fn project(&self, point: Point<PRCS, f64>) -> Result<(f64, f64)> {
        let cmcs = self.mount_calibration.matrix() *
                   try!(self.cop.inverse().ok_or(Error::MissingInverse(self.cop))) *
                   try!(self.sop.inverse().ok_or(Error::MissingInverse(self.sop))) *
                   Vector4::from(point);
        assert!(cmcs.w == 1.);
        let cmcs = Vector3::new(cmcs.x, cmcs.y, cmcs.z);
        match self.camera_calibration {
            CameraCalibration::OpenCv { internal_opencv: cam, version, .. } => {
                let a = Matrix3::new(cam.fx, 0., cam.cx, 0., cam.fy, cam.cy, 0., 0., 1.);
                let ud_prime = a * cmcs;
                let u = ud_prime[0] / ud_prime[2];
                let v = ud_prime[1] / ud_prime[2];
                let x = (u - cam.cx) / cam.fx;
                let y = (v - cam.cy) / cam.fy;
                let r = match version {
                    2 => (x * x + y * y).sqrt().atan().powi(2).sqrt(),
                    _ => return Err(Error::UnsupportedOpenCvVersion(version)),
                };
                let expansion = cam.k1 * r.powi(2) + cam.k2 * r.powi(4) + cam.k3 * r.powi(6) +
                                cam.k4 * r.powi(8);
                let ud = u + x * cam.fx * expansion + 2. * cam.fx * x * y * cam.p1 +
                         cam.p2 * cam.fx * (r.powi(2) + 2. * x.powi(2));
                let vd = v + y * cam.fy * expansion + 2. * cam.fy * x * y * cam.p2 +
                         cam.p1 * cam.fy * (r.powi(2) + 2. * y.powi(2));
                Ok((ud, vd))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MountCalibration {
    matrix: Matrix4<f64>,
    name: String,
}

impl MountCalibration {
    fn from_element(element: &Element) -> Result<MountCalibration> {
        Ok(MountCalibration {
            matrix: try!(element.get_matrix4("matrix")),
            name: try!(element.get_text("name")).to_string(),
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn matrix(&self) -> Matrix4<f64> {
        self.matrix
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
    #[ignore]
    fn scan_position_color() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position("SP01").unwrap();
        assert_eq!(22.49, scan_position.color(point()).unwrap().unwrap());
    }
}
