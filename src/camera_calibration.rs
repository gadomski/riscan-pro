use {Cmcs, Point, Result};
use std::path::Path;

/// A camera calibration.
///
/// Only opencv camera calibrations are supported at this time.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[allow(missing_docs)]
pub struct CameraCalibration {
    /// The name of the calibration.
    pub name: String,
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
    pub tan_max_horz: f64,
    pub tan_max_vert: f64,
    pub tan_min_horz: f64,
    pub tan_min_vert: f64,
    pub width: usize,
    pub height: usize,
}

impl CameraCalibration {
    /// Retrieves all camera calibrations from a project.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::CameraCalibration;
    /// let camera_calibrations = CameraCalibration::from_project_path("data/project.RiSCAN")
    ///     .unwrap();
    /// assert_eq!(1, camera_calibrations.len());
    /// ```
    pub fn from_project_path<P: AsRef<Path>>(path: P) -> Result<Vec<CameraCalibration>> {
        use Project;
        let project = Project::from_path(path)?;
        Ok(project.camera_calibrations.values().cloned().collect())
    }

    /// Converts a point in the camera's coordinate system to pixel values.
    ///
    /// The pixel values are floats, in case someone later wants to do more than a direct lookup.
    ///
    /// Returns None if:
    ///
    /// - The point is behind the camera (negative z).
    /// - The point is ouside the angle extents, as defined by `tan_{min|max}_{vert|horz}`.
    /// - The calculated pixel values are outside of the width/height of the image.
    ///
    /// These maths are taken from the `project.dtd` file in every RiSCAN Pro project.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{CameraCalibration, Point};
    /// let camera_calibration = CameraCalibration::from_project_path("data/southpole.rsp")
    ///     .unwrap()
    ///     .pop()
    ///     .unwrap();
    /// let cmcs = Point::cmcs(1.312, -0.641, 3.019);
    /// let (u, v) = camera_calibration.cmcs_to_ics(&cmcs).unwrap();
    /// ```
    pub fn cmcs_to_ics(&self, point: &Point<Cmcs>) -> Option<(f64, f64)> {
        use nalgebra::Matrix3;
        use std::ops::Deref;

        if point.is_behind_camera() {
            return None;
        }

        let tan_horz = point.tan_horz();
        let tan_vert = point.tan_vert();
        if tan_horz < self.tan_min_horz || tan_horz > self.tan_max_horz ||
            tan_vert < self.tan_min_vert || tan_vert > self.tan_max_vert
        {
            return None;
        }

        let a = Matrix3::new(self.fx, 0., self.cx, 0., self.fy, self.cy, 0., 0., 1.);
        let ud_prime = a * point.deref();
        let u = ud_prime[0] / ud_prime[2];
        let v = ud_prime[1] / ud_prime[2];
        let x = (u - self.cx) / self.fx;
        let y = (v - self.cy) / self.fy;
        let r = (x.powi(2) + y.powi(2)).sqrt().atan().powi(2).sqrt();
        let r_term = self.k1 * r.powi(2) + self.k2 * r.powi(4) + self.k3 * r.powi(6) +
            self.k4 * r.powi(8);
        let u = u + x * self.fx * r_term + 2. * self.fx * x * y * self.p1 +
            self.p2 * self.fx * (r.powi(2) + 2. * x.powi(2));
        let v = v + y * self.fy * r_term + 2. * self.fy * x * y * self.p2 +
            self.p1 * self.fy * (r.powi(2) + 2. * y.powi(2));

        if self.is_valid_pixel(u, v) {
            Some((u, v))
        } else {
            None
        }
    }

    /// Returns true if this is a valid pixel value.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::CameraCalibration;
    /// let camera_calibration = CameraCalibration::from_project_path("data/project.RiSCAN")
    ///     .unwrap()
    ///     .pop()
    ///     .unwrap();
    /// // The camera calibration is 1024x768
    /// assert!(camera_calibration.is_valid_pixel(0., 0.));
    /// assert!(!camera_calibration.is_valid_pixel(1024., 0.));
    /// assert!(!camera_calibration.is_valid_pixel(0., 768.));
    /// ```
    pub fn is_valid_pixel<T: Into<f64>>(&self, u: T, v: T) -> bool {
        let u = u.into();
        let v = v.into();
        u >= 0. && v >= 0. && u < self.width as f64 && v < self.height as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmcs_to_ics() {
        let camera_calibration = CameraCalibration::from_project_path("data/southpole.rsp")
            .unwrap()
            .pop()
            .unwrap();
        let cmcs = Point::cmcs(1.312, -0.641, 3.019);
        let (u, v) = camera_calibration.cmcs_to_ics(&cmcs).unwrap();
        assert_relative_eq!(882.668, u, epsilon = 1e-3);
        assert_relative_eq!(228.443, v, epsilon = 1e-3);

        // Point is *way* low.
        let cmcs = Point::cmcs(-100., -0.641, 3.019);
        assert_eq!(None, camera_calibration.cmcs_to_ics(&cmcs));
        // Point is behind camera.
        let cmcs = Point::cmcs(1.312, -0.641, -3.019);
        assert_eq!(None, camera_calibration.cmcs_to_ics(&cmcs));
    }

    #[test]
    fn is_valid_pixel() {
        let camera_calibration = CameraCalibration::from_project_path("data/southpole.rsp")
            .unwrap()
            .pop()
            .unwrap();
        assert!(camera_calibration.is_valid_pixel(0, 0));
        assert!(!camera_calibration.is_valid_pixel(-1, 0));
        assert!(!camera_calibration.is_valid_pixel(0, -1));
        assert!(!camera_calibration.is_valid_pixel(1024, 0));
        assert!(!camera_calibration.is_valid_pixel(0, 768));
        assert!(camera_calibration.is_valid_pixel(1023.9, 0.));
        assert!(camera_calibration.is_valid_pixel(0., 767.9));
    }
}
