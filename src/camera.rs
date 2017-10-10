use Result;
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
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub fx: f64,
    pub fy: f64,
    pub cx: f64,
    pub cy: f64,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub k4: f64,
    pub p1: f64,
    pub p2: f64,
    pub nx: usize,
    pub ny: usize,
    pub dx: f64,
    pub dy: f64,
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
}
