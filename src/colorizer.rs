use {CameraCalibration, Cmcs, MountCalibration, Point, Result, Socs, scan_position};
use irb;
use std::path::Path;

/// Takes in points and returns the color for that point.
#[derive(Debug)]
pub struct Colorizer {
    camera_calibration: CameraCalibration,
    image: scan_position::Image,
    irb: irb::text::Irb,
    mount_calibration: MountCalibration,
}

impl Colorizer {
    /// Creates a colorizer for the provided path.
    ///
    /// The path must contain enough information to intuit the project, scan position, and project
    /// image name.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Colorizer;
    /// let path = "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv";
    /// let colorizer = Colorizer::from_path(path).unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Colorizer> {
        use {Error, Project};
        use irb::text::Irb;

        let project = Project::from_path(&path)?;
        let image = project.image_from_path(&path)?;
        let camera_calibration = project
            .camera_calibrations
            .get(&image.camera_calibration_name)
            .ok_or(Error::MissingCameraCalibration(
                image.camera_calibration_name.clone(),
            ))?;
        let mount_calibration = project
            .mount_calibrations
            .get(&image.mount_calibration_name)
            .ok_or(Error::MissingMountCalibration(
                image.mount_calibration_name.clone(),
            ))?;
        let irb = Irb::from_path(path)?;
        Ok(Colorizer {
            camera_calibration: camera_calibration.clone(),
            image: image.clone(),
            irb: irb,
            mount_calibration: mount_calibration.clone(),
        })
    }

    /// Return the camera's coordinates for a point in the scanner's own coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Colorizer, Point};
    /// let colorizer = Colorizer::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv")
    ///     .unwrap();
    /// let socs = Point::socs(10., -5.0, 2.0);
    /// let cmcs = colorizer.socs_to_cmcs(&socs);
    /// ```
    pub fn socs_to_cmcs(&self, point: &Point<Socs>) -> Point<Cmcs> {
        use std::ops::Deref;
        (*self.mount_calibration * self.image.cop.inverse() * point.deref()).into()
    }

    /// Return the pixel coordinates for a point in the scanner's own coordinate system.
    ///
    /// Returns none if the coordinates are not in the image view.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Colorizer, Point};
    /// let colorizer = Colorizer::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv")
    ///     .unwrap();
    /// let coordinate = Point::socs(-7.429, 6.834, 0.076);
    /// let (u, v) = colorizer.pixel(&coordinate).unwrap();
    /// ```
    pub fn pixel(&self, point: &Point<Socs>) -> Option<(f64, f64)> {
        let cmcs = self.socs_to_cmcs(point);
        self.camera_calibration.cmcs_to_ics(&cmcs)
    }

    /// Colorize a point provided in the scanner's own coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Colorizer, Point};
    /// let colorizer = Colorizer::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv")
    ///     .unwrap();
    /// let coordinate = Point::socs(-7.429, 6.834, 0.076);
    /// let color = colorizer.colorize(&coordinate).unwrap();
    /// ```
    pub fn colorize(&self, point: &Point<Socs>) -> Option<f64> {
        self.pixel(point).and_then(|(u, v)| {
            assert!(u >= 0.);
            assert!(v >= 0.);
            self.irb
                .temperature(u.trunc() as usize, v.trunc() as usize)
                .map(|&n| n)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_path() {
        Colorizer::from_path(
            "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv",
        ).unwrap();
        assert!(
            Colorizer::from_path(
                "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP02 - Image001.csv",
            ).is_err()
        );
    }

    #[test]
    fn colorize() {
        let colorizer = Colorizer::from_path(
            "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv",
        ).unwrap();
        let coordinate = Point::socs(-7.429, 6.834, 0.076);
        let color = colorizer.colorize(&coordinate).unwrap();
        assert_eq!(24.46, color);
    }
}
