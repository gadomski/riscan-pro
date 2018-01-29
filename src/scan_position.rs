//! Scan positions and their consituant parts.

use {CameraCalibration, Error, MountCalibration, Project, Result};
use nalgebra::Projective3;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A scan position
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ScanPosition {
    /// The name of the scan position.
    pub name: String,
    /// The scan position images.
    pub images: BTreeMap<String, Image>,
    /// The scanner's own position.
    pub sop: Projective3<f64>,
    /// The scans taken at this position.
    pub scans: BTreeMap<String, Scan>,
    /// The scan position SOP matrix is frozen.
    pub is_frozen: bool,
}

/// A scan.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Scan {
    /// The name of the scan.
    pub name: String,
    /// The file name of the scan.
    pub file: String,
    /// The number of measurements in the phi direction.
    pub phi_count: usize,
    /// The number of measurements in the theta direction.
    pub theta_count: usize,
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
            .ok_or_else(|| Error::ImageFromPath(path.as_ref().to_path_buf()))
    }

    /// Returns a vector of all paths to rxps in the singlescan directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = project
    ///     .scan_positions
    ///     .get("SP01")
    ///     .unwrap();
    /// let paths = scan_position.singlescan_rxp_paths(&project);
    /// assert_eq!(4, paths.len());
    /// ```
    pub fn singlescan_rxp_paths(&self, project: &Project) -> Vec<PathBuf> {
        let mut path = project
            .path
            .parent()
            .expect("Project path should always have a parent")
            .to_path_buf();
        path.push("SCANS");
        path.push(&self.name);
        path.push("SINGLESCANS");
        self.scans
            .values()
            .map(|scan| {
                let mut path = path.clone();
                path.push(&scan.file);
                path
            })
            .collect()
    }

    /// Returns a vector of all of the images.
    ///
    /// The vector is sorted by name.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = &project.scan_positions["SP01"];
    /// let images = scan_position.images();
    /// assert_eq!(6, images.len());
    /// ```
    pub fn images(&self) -> Vec<&Image> {
        let mut images: Vec<_> = self.images.values().collect();
        images.sort_by_key(|i| &i.name);
        images
    }
}

impl Image {
    /// Finds and returns this image's camera calibration.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let mut image = project.scan_positions
    ///     .get("SP01")
    ///     .unwrap()
    ///     .images
    ///     .get("SP01 - Image001")
    ///     .unwrap()
    ///     .clone();
    /// image.camera_calibration(&project).unwrap();
    /// image.camera_calibration_name = "Not a camera calibration".to_string();
    /// assert!(image.camera_calibration(&project).is_err());
    /// ```
    pub fn camera_calibration<'a>(&self, project: &'a Project) -> Result<&'a CameraCalibration> {
        project
            .camera_calibrations
            .get(&self.camera_calibration_name)
            .ok_or_else(|| {
                Error::MissingCameraCalibration(self.camera_calibration_name.clone())
            })
    }

    /// Finds and returns this image's mount calibration.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Project;
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let mut image = project.scan_positions
    ///     .get("SP01")
    ///     .unwrap()
    ///     .images
    ///     .get("SP01 - Image001")
    ///     .unwrap()
    ///     .clone();
    /// image.mount_calibration(&project).unwrap();
    /// image.mount_calibration_name = "Not a mount calibration".to_string();
    /// assert!(image.mount_calibration(&project).is_err());
    /// ```
    pub fn mount_calibration<'a>(&self, project: &'a Project) -> Result<&'a MountCalibration> {
        project
            .mount_calibrations
            .get(&self.mount_calibration_name)
            .ok_or_else(|| {
                Error::MissingMountCalibration(self.mount_calibration_name.clone())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_position_singlescan_rxp_paths() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_positions.get("SP01").unwrap();
        let paths = scan_position.singlescan_rxp_paths(&project);
        assert_eq!(4, paths.len());
    }
}
