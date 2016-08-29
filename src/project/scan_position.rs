use std::collections::HashMap;

use Result;
use point::{PRCS, Point};
use project::{Image, Scan};

/// A fixed postion where one or more scans were taken, along with optional pictures and other data.
#[derive(Debug, PartialEq)]
pub struct ScanPosition {
    name: String,
    images: HashMap<String, Image>,
    scans: HashMap<String, Scan>,
}

impl ScanPosition {
    /// Creates a new scan position with the given name, images, and scans.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::collections::HashMap;
    /// # use riscan_pro::ScanPosition;
    /// let scan_position = ScanPosition::new("ScanPos001",
    ///                                       HashMap::new(),
    ///                                       HashMap::new());
    /// ```
    pub fn new(name: &str,
               images: HashMap<String, Image>,
               scans: HashMap<String, Scan>)
               -> ScanPosition {
        ScanPosition {
            name: name.to_string(),
            images: images,
            scans: scans,
        }
    }

    /// Returns this scan position's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = project.scan_position("SP01").unwrap();
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if the scan position contains a scan of the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let scan_position = project.scan_position("SP01").unwrap();
    /// assert!(scan_position.contains_scan("151120_150227"));
    /// assert!(!scan_position.contains_scan("151120_150228"));
    /// ```
    pub fn contains_scan(&self, name: &str) -> bool {
        self.scans.contains_key(name)
    }

    /// Returns a vector of references to all images that do not have data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// # let scan_position = project.scan_position("SP01").unwrap();
    /// let missing_images = scan_position.missing_images();
    /// ```
    pub fn missing_images(&self) -> Vec<&Image> {
        self.images.values().filter(|i| !i.has_data()).collect()
    }

    /// Computes the color for a given PRCS point from all contained images.
    ///
    /// If an image does not have any associated data, it is skipped.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{Project, Point, PRCS};
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// # let scan_position = project.scan_position("SP01").unwrap();
    /// let point = Point {
    ///     crs: PRCS,
    ///     x: -139.31727,
    ///     y: -239.32973,
    ///     z: -10.49305,
    /// };
    /// let color = scan_position.color(point).unwrap().unwrap();
    /// ```
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

    /// Returns the image of the given name in this scan position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// # let scan_position = project.scan_position("SP01").unwrap();
    /// let image = scan_position.image("SP01 - Image001").unwrap();
    /// ```
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
