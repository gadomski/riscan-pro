use {Cmcs, Point};

/// A camera calibration.
///
/// Thesee are opencv camera definitions, specififed by Riegl.
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
    /// Converts a point in the camera's coordinate system to pixel values.
    ///
    /// If the incoming point is outside of the angle extents, or the resultant pixel is outside of
    /// the image bounds, returns None.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    /// use riscan_pro::{CameraCalibration, Point};
    /// let camera_calibration = CameraCalibration::from_path("data/camera.cam").unwrap();
    /// let cmcs = Point::cmcs(10., -5., 2.);
    /// let (u, v) = camera_calibration.cmcs_to_ics(cmcs);
    /// ```
    pub fn cmcs_to_ics(&self, point: &Point<Cmcs>) -> Option<(f64, f64)> {
        use nalgebra::Matrix3;
        use std::ops::Deref;

        // Point is behind the camera.
        if point.z < 0. {
            return None;
        }

        let tan_horz = point.y / point.z;
        let tan_vert = point.x / point.z;
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

        if u < 0. || u.trunc() as usize > self.width || v < 0. || v.trunc() as usize > self.height {
            None
        } else {
            Some((u, v))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Project;

    #[test]
    fn cmcs_to_ics() {
        let project = Project::from_path("data/southpole.rsp").unwrap();
        let camera_calibration = project
            .camera_calibrations
            .get(
                "Result calibration (Infratec_VarioCAM_HD_15mm_11-16-2015_Preston)_1024x768",
            )
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
}
