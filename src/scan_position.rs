//! Scan positions and their consituant parts.

use nalgebra::Projective3;
use std::collections::HashMap;

/// A scan position
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ScanPosition {
    /// The name of the scan position.
    pub name: String,
    /// The scan position images.
    pub images: HashMap<String, Image>,
}

/// A scan position image.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Image {
    /// The name of the image.
    pub name: String,
    /// The camera's own position when taking the image.
    pub cop: Projective3<f64>,
    /// The name of the image's camera calibration.
    pub camera_calibration_name: String,
    /// The name of the image's mount calibration.
    pub mount_calibration_name: String,
}
