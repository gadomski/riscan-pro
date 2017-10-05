use {Point3, Result};
use element::{Extension, FromElement};
use std::path::Path;
use xmltree::Element;

macro_rules! setting {
    ($settings:expr, $name:expr) => {{
        use Error;
        *$settings.get($name).ok_or(Error::MissingCameraSetting($name.to_string()))?
    }}
}

/// An opencv camera calibration.
///
/// These can be stored in a project rsp file or in a seperate calibration file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    fx: f64,
    fy: f64,
    cx: f64,
    cy: f64,
    k1: f64,
    k2: f64,
    k3: f64,
    k4: f64,
    p1: f64,
    p2: f64,
    nx: usize,
    ny: usize,
    dx: f64,
    dy: f64,
}

impl Camera {
    /// Reads a camera calibration from a path.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Camera;
    /// let camera = Camera::from_path("data/camera.cam").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Camera> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        use std::collections::HashMap;

        let settings: HashMap<String, f64> = BufReader::new(File::open(path)?)
            .lines()
            .filter_map(|line| {
                line.ok().and_then(|line| {
                    let words = line.split('=').collect::<Vec<_>>();
                    if words.len() == 2 {
                        words[1].parse().ok().map(|n| (words[0].to_string().to_lowercase(), n))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(Camera {
               fx: setting!(settings, "fx"),
               fy: setting!(settings, "fy"),
               cx: setting!(settings, "cx"),
               cy: setting!(settings, "cy"),
               k1: setting!(settings, "k1"),
               k2: setting!(settings, "k2"),
               k3: setting!(settings, "k3"),
               k4: setting!(settings, "k4"),
               p1: setting!(settings, "p1"),
               p2: setting!(settings, "p2"),
               nx: setting!(settings, "nx") as usize,
               ny: setting!(settings, "ny") as usize,
               dx: setting!(settings, "dx"),
               dy: setting!(settings, "dy"),
           })
    }

    /// Convert camera's coordinates to image coordinate space.
    ///
    /// Returns the distorted coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Camera, Point3};
    /// let camera = Camera::from_path("data/camera.cam").unwrap();
    /// let point = Point3::new(1., 2., 3.);
    /// let (u, v) = camera.cmcs_to_ics(point);
    /// ```
    pub fn cmcs_to_ics(&self, point: Point3) -> (f64, f64) {
        use nalgebra::Matrix3;

        let a = Matrix3::new(self.fx, 0., self.cx, 0., self.fy, self.cy, 0., 0., 1.);
        let ud_prime = a * point;
        let u = ud_prime[0] / ud_prime[2];
        let v = ud_prime[1] / ud_prime[2];
        let x = (u - self.cx) / self.fx;
        let y = (v - self.cy) / self.fy;
        let r = (x.powi(2) + y.powi(2))
            .sqrt()
            .atan()
            .powi(2)
            .sqrt();
        let r_term = self.k1 * r.powi(2) + self.k2 * r.powi(4) + self.k3 * r.powi(6) +
                     self.k4 * r.powi(8);
        let u = u + x * self.fx * r_term + 2. * self.fx * x * y * self.p1 +
                self.p2 * self.fx * (r.powi(2) + 2. * x.powi(2));
        let v = v + y * self.fy * r_term + 2. * self.fy * x * y * self.p2 +
                self.p1 * self.fy * (r.powi(2) + 2. * y.powi(2));
        (u, v)
    }
}

impl FromElement for Camera {
    fn from_element(element: &Element) -> Result<Camera> {
        Ok(Camera {
               fx: element.xpath_then_parse("internal_opencv/fx")?,
               fy: element.xpath_then_parse("internal_opencv/fy")?,
               cx: element.xpath_then_parse("internal_opencv/cx")?,
               cy: element.xpath_then_parse("internal_opencv/cy")?,
               k1: element.xpath_then_parse("internal_opencv/k1")?,
               k2: element.xpath_then_parse("internal_opencv/k2")?,
               k3: element.xpath_then_parse("internal_opencv/k3")?,
               k4: element.xpath_then_parse("internal_opencv/k4")?,
               p1: element.xpath_then_parse("internal_opencv/p1")?,
               p2: element.xpath_then_parse("internal_opencv/p2")?,
               nx: element.xpath_then_parse("intrinsic_opencv/nx")?,
               ny: element.xpath_then_parse("intrinsic_opencv/ny")?,
               dx: element.xpath_then_parse("intrinsic_opencv/dx")?,
               dy: element.xpath_then_parse("intrinsic_opencv/dy")?,
           })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Point3;

    #[test]
    fn from_path() {
        let camera = Camera::from_path("data/camera.cam").unwrap();
        assert_relative_eq!(883.5230667826110700, camera.fx);
        assert_relative_eq!(884.2400341059731200, camera.fy);
        assert_relative_eq!(-0.4164950117338272, camera.k1);
        assert_relative_eq!(0.1101340143848572, camera.k2);
        assert_relative_eq!(-0.1163151291620893, camera.k3);
        assert_relative_eq!(0.0141611215992979, camera.k4);
        assert_relative_eq!(529.9796605360121600, camera.cx);
        assert_relative_eq!(400.7770650611840000, camera.cy);
        assert_relative_eq!(0.0007262701857370, camera.p1);
        assert_relative_eq!(-0.0001515209871080, camera.p2);
        assert_eq!(1024, camera.nx);
        assert_eq!(768, camera.ny);
        assert_relative_eq!(0.0000170000000000, camera.dx);
        assert_relative_eq!(0.0000170000000000, camera.dy);
    }

    #[test]
    fn cmcs_to_ics() {
        let point = Point3::new(1., 2., 3.);
        let camera = Camera::from_path("data/camera.cam").unwrap();
        let (u, v) = camera.cmcs_to_ics(point);
        assert_relative_eq!(777.5760, u, epsilon = 1e-4);
        assert_relative_eq!(896.7450, v, epsilon = 1e-4);
    }
}
