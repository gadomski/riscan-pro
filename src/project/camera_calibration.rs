use xmltree::Element;

use {Error, Result};
use project::traits::GetDescendant;

#[derive(Clone, Debug, PartialEq)]
pub enum CameraCalibration {
    OpenCv {
        name: String,
        cameramodel: String,
        version: u8,
        angle_extents: opencv::AngleExtents,
        internal_opencv: opencv::Internal,
        intrinsic_opencv: opencv::Intrinsic,
    },
}

impl CameraCalibration {
    pub fn from_element(element: &Element) -> Result<CameraCalibration> {
        match element.name.as_str() {
            "camcalib_opencv" => {
                Ok(CameraCalibration::OpenCv {
                    name: try!(element.get_text("name")).to_string(),
                    cameramodel: try!(element.get_text("cameramodel")).to_string(),
                    version: try!(element.parse("version")),
                    angle_extents: opencv::AngleExtents {
                        tan_max_horz: try!(element.parse("angle_extents/tan_max_horz")),
                        tan_max_vert: try!(element.parse("angle_extents/tan_max_vert")),
                        tan_min_horz: try!(element.parse("angle_extents/tan_min_horz")),
                        tan_min_vert: try!(element.parse("angle_extents/tan_min_vert")),
                    },
                    internal_opencv: opencv::Internal {
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
                    intrinsic_opencv: opencv::Intrinsic {
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

    pub fn name(&self) -> &str {
        match *self {
            CameraCalibration::OpenCv { ref name, .. } => name,
        }
    }
}

pub mod opencv {
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(missing_docs)]
    pub struct AngleExtents {
        pub tan_max_horz: f64,
        pub tan_max_vert: f64,
        pub tan_min_horz: f64,
        pub tan_min_vert: f64,
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(missing_docs)]
    pub struct Internal {
        pub cx: f64,
        pub cy: f64,
        pub fx: f64,
        pub fy: f64,
        pub k1: f64,
        pub k2: f64,
        pub k3: f64,
        pub k4: f64,
        pub p1: f64,
        pub p2: f64,
    }
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[allow(missing_docs)]
    pub struct Intrinsic {
        pub dx: f64,
        pub dy: f64,
        pub nx: usize,
        pub ny: usize,
    }
}
