use std::collections::HashMap;

use xmltree::Element;

use {Error, Result};
use point::{PRCS, Point};
use project::{CameraCalibration, Image, MountCalibration};
use project::traits::GetDescendant;

/// A fixed postion where one or more scans were taken, along with optional pictures and other data.
#[derive(Debug, PartialEq)]
pub struct ScanPosition {
    name: String,
    images: HashMap<String, Image>,
}

impl ScanPosition {
    pub fn from_element(element: &Element,
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
