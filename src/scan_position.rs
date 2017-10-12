//! Scan positions and their consituant parts.

use Result;
use nalgebra::Projective3;
use std::collections::HashMap;
use std::path::Path;

/// A scan position
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ScanPosition {
    /// The name of the scan position.
    pub name: String,
    /// The scan position images.
    pub images: HashMap<String, Image>,
    /// The scanner's own position.
    pub sop: Projective3<f64>,
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

impl ScanPosition {
    /// Returns a scan position image, as determined by the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = project.scan_positions.get("SP01").unwrap();
    /// let image1 = scan_position.images.get("SP01 - Image001").unwrap();
    /// let path = "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv";
    /// let image2 = scan_position.image_from_path(path).unwrap();
    /// assert_eq!(image1, image2);
    /// ```
    pub fn image_from_path<P: AsRef<Path>>(&self, path: P) -> Result<&Image> {
        use Error;
        path.as_ref()
            .file_stem()
            .map(|file_stem| file_stem.to_string_lossy())
            .and_then(|file_stem| self.images.get(file_stem.as_ref()))
            .ok_or(Error::ImageFromPath(path.as_ref().to_path_buf()))
    }
}
