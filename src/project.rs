use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

use xmltree::Element;

use {Error, Result};

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
        let scan_positions: HashMap<String, ScanPosition> = try!(xml.get_children("scanpositions")
            .and_then(|children| {
                children.iter()
                    .map(|child| {
                        let scan_position = try!(ScanPosition::from_element(child));
                        Ok((scan_position.name().to_string(), scan_position))
                    })
                    .collect()
            }));
        Ok(Project { scan_positions: scan_positions })
    }

    /// Returns scan position with the provided name.
    ///
    /// If the scan position does not exist in the project, returns `Ok(None)`.
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
}

impl ScanPosition {
    fn from_element(element: &Element) -> Result<ScanPosition> {
        Ok(ScanPosition { name: try!(element.get_text("name")).to_string() })
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn color(&self, x: f64, y: f64, z: f64) -> Option<f64> {
        for image in self.images() {
            let color = image.color(x, y, z);
            if color.is_some() {
                return color;
            }
        }
        None
    }

    fn images(&self) -> &Vec<Image> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct Image;

impl Image {
    fn color(&self, x: f64, y: f64, z: f64) -> Option<f64> {
        unimplemented!()
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

    #[test]
    fn project_scan_position() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        assert!(project.scan_position("SP01").is_some());
        assert!(project.scan_position("SP02").is_some());
        assert!(project.scan_position("SP03").is_none());
    }

    #[test]
    fn scan_position_color() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position("SP01").unwrap();
        assert_eq!(22.49,
                   scan_position.color(-139.31727, -239.32973, -10.49305).unwrap());
    }
}
