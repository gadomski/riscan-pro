use std::collections::HashMap;
use std::path::Path;

use xmltree::Element;

use {Error, Result};
use point::{PRCS, Point};
use project::{CameraCalibration, Image, MountCalibration, Scan};
use project::traits::GetDescendant;

/// A fixed postion where one or more scans were taken, along with optional pictures and other data.
#[derive(Debug, PartialEq)]
pub struct ScanPosition {
    name: String,
    images: HashMap<String, Image>,
    scans: HashMap<String, Scan>,
}

impl ScanPosition {
    pub fn from_element<P>(element: &Element,
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
        Ok(ScanPosition {
            name: name.to_string(),
            images: images,
            scans: scans,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contains_scan(&self, name: &str) -> bool {
        self.scans.contains_key(name)
    }

    pub fn missing_images(&self) -> Vec<&Image> {
        self.images.values().filter(|i| !i.has_data()).collect()
    }

    pub fn color(&self, point: Point<PRCS, f64>) -> Result<Option<f64>> {
        for image in self.images.values() {
            if !image.has_data() {
                continue;
            }
            let color = try!(image.color(point));
            if color.is_some() {
                return Ok(color);
            }
        }
        Ok(None)
    }

    pub fn image(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}

#[cfg(test)]
mod test {
    use project::Project;

    #[test]
    fn scan_position_missing_images() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position("SP01").unwrap();
        let images = scan_position.missing_images();
        assert_eq!(5, images.len());
    }
}
