/// Camera calibration to turn points into pixels.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum CameraCalibration {
    /// OpenCv camera calibration.
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
    /// Returns this camera calibration's name.
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
