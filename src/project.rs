use std::collections::HashMap;
use std::fs::{self, File};
use std::iter::FromIterator;
use std::path::Path;

use nalgebra::{Inverse, Matrix4, Vector4};
use xmltree::Element;

use {Error, Result};
use point::{PRCS, Point};
use utils;

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
        let scan_positions = try!(xml.map_children("scanpositions", |child| {
            let scan_position = try!(ScanPosition::from_element(child, &mount_calibrations));
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

trait GetDescendant {
    fn get_descendant(&self, name: &str) -> Result<&Element>;
    fn get_children(&self, name: &str) -> Result<&Vec<Element>> {
        self.get_descendant(name).map(|e| &e.children)
    }
    fn get_text(&self, name: &str) -> Result<&str> {
        self.get_descendant(name)
            .and_then(|e| {
                e.text.as_ref().map(|s| s.as_str()).ok_or(Error::MissingElement(name.to_string()))
            })
    }
    fn get_noderef(&self, name: &str) -> Result<&str> {
        self.get_descendant(name).and_then(|e| {
            e.attributes
                .get("noderef")
                .and_then(|s| s.split('/').last())
                .ok_or(Error::MissingElement(format!("{}.noderef", name)))
        })
    }
    fn get_matrix4(&self, name: &str) -> Result<Matrix4<f64>> {
        self.get_text(name).and_then(|s| utils::matrix4_from_str(s))
    }
    fn map_children<F, A, B>(&self, name: &str, function: F) -> Result<A>
        where F: Fn(&Element) -> Result<B>,
              A: FromIterator<B>
    {
        self.get_children(name).and_then(|children| {
            children.iter()
                .map(function)
                .collect()
        })
    }
}

impl GetDescendant for Element {
    fn get_descendant(&self, name: &str) -> Result<&Element> {
        let mut element = self;
        for name in name.split('/') {
            match element.get_child(name) {
                Some(child) => element = child,
                None => return Err(Error::MissingElement(name.to_string())),
            }
        }
        Ok(element)
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
                    mount_calibrations: &HashMap<String, MountCalibration>)
                    -> Result<ScanPosition> {
        let sop = try!(element.get_matrix4("sop/matrix"));
        let images = try!(element.map_children("scanposimages", |child| {
            let ref mount_calibration = try!(child.get_noderef("mountcalib_ref")
                .and_then(|name| {
                    mount_calibrations.get(name)
                        .ok_or(Error::MissingElement(format!("mount_calibration[name={}]", name)))
                }));
            let image = try!(Image::from_element(child, (*mount_calibration).clone(), sop));
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
    mount_calibration: MountCalibration,
    name: String,
    sop: Matrix4<f64>,
}

impl Image {
    fn from_element(element: &Element,
                    mount_calibration: MountCalibration,
                    sop: Matrix4<f64>)
                    -> Result<Image> {
        Ok(Image {
            name: try!(element.get_text("name")).to_string(),
            mount_calibration: mount_calibration,
            cop: try!(element.get_matrix4("cop/matrix")),
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
        unimplemented!()
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
