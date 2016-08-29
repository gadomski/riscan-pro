use std::fmt;
use std::fs;
use std::path::Path;

use nalgebra::{Inverse, Matrix3, Matrix4, Vector3, Vector4};

use {Error, Result};
use infratec;
use point::{PRCS, Point};
use project::{CameraCalibration, MountCalibration};

/// A project image.
///
/// This struct may or may not be associated with actual image data. Image data are loaded into an
/// `Image` via a magical discovery process.
#[derive(Debug, PartialEq)]
pub struct Image {
    cop: Matrix4<f64>,
    camera_calibration: CameraCalibration,
    image_data: Option<Box<ImageData>>,
    mount_calibration: MountCalibration,
    name: String,
    scan_position_name: String,
    sop: Matrix4<f64>,
}

impl Image {
    /// Creates a new image.
    pub fn new(scan_position_name: &str,
               sop: Matrix4<f64>,
               name: &str,
               mount_calibration: MountCalibration,
               camera_calibration: CameraCalibration,
               cop: Matrix4<f64>,
               image_data: Option<Box<ImageData>>)
               -> Image {
        Image {
            cop: cop,
            camera_calibration: camera_calibration,
            image_data: image_data,
            mount_calibration: mount_calibration,
            name: name.to_string(),
            scan_position_name: scan_position_name.to_string(),
            sop: sop,
        }
    }

    /// Returns this image's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// # let image = project.image("SP01", "SP01 - Image001").unwrap();
    /// let name = image.name();
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the name of the scan position that holds this image.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Project;
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// # let image = project.image("SP01", "SP01 - Image001").unwrap();
    /// let name = image.scan_position_name();
    /// ```
    pub fn scan_position_name(&self) -> &str {
        &self.scan_position_name
    }

    /// Computes the color value associated with this image at the given PRCS coordinates.
    ///
    /// For now, "color" is just a single pixel value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{Project, Point, PRCS};
    /// # let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let image = project.image("SP01", "SP01 - Image001").unwrap();
    /// let point = Point {
    ///     crs: PRCS,
    ///     x: -139.31727,
    ///     y: -239.32973,
    ///     z: -10.49305,
    /// };
    /// let color = image.color(point).unwrap().unwrap();
    /// ```
    pub fn color(&self, point: Point<PRCS, f64>) -> Result<Option<f64>> {
        let data = try!(self.image_data
            .as_ref()
            .ok_or(Error::MissingImageData(self.scan_position_name().to_string(),
                                           self.name().to_string())));
        let (u, v) = try!(self.project(point));
        Ok(data.get(u, v))
    }

    fn project(&self, point: Point<PRCS, f64>) -> Result<(f64, f64)> {
        let cmcs = self.mount_calibration.matrix() *
                   try!(self.cop.inverse().ok_or(Error::MissingInverse(self.cop))) *
                   try!(self.sop.inverse().ok_or(Error::MissingInverse(self.sop))) *
                   Vector4::from(point);
        assert!(cmcs.w == 1.);
        let cmcs = Vector3::new(cmcs.x, cmcs.y, cmcs.z);
        match self.camera_calibration {
            CameraCalibration::OpenCv { internal_opencv: cam, version, .. } => {
                let a = Matrix3::new(cam.fx, 0., cam.cx, 0., cam.fy, cam.cy, 0., 0., 1.);
                let ud_prime = a * cmcs;
                let u = ud_prime[0] / ud_prime[2];
                let v = ud_prime[1] / ud_prime[2];
                let x = (u - cam.cx) / cam.fx;
                let y = (v - cam.cy) / cam.fy;
                let r = match version {
                    2 => (x * x + y * y).sqrt().atan().powi(2).sqrt(),
                    _ => return Err(Error::UnsupportedOpenCvVersion(version)),
                };
                let expansion = cam.k1 * r.powi(2) + cam.k2 * r.powi(4) + cam.k3 * r.powi(6) +
                                cam.k4 * r.powi(8);
                let ud = u + x * cam.fx * expansion + 2. * cam.fx * x * y * cam.p1 +
                         cam.p2 * cam.fx * (r.powi(2) + 2. * x.powi(2));
                let vd = v + y * cam.fy * expansion + 2. * cam.fy * x * y * cam.p2 +
                         cam.p1 * cam.fy * (r.powi(2) + 2. * y.powi(2));
                Ok((ud, vd))
            }
        }
    }

    /// Returns true if this image has associated data.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Project, Point, PRCS};
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let image1 = project.image("SP01", "SP01 - Image001").unwrap();
    /// assert!(image1.has_data());
    /// let image2 = project.image("SP01", "SP01 - Image002").unwrap();
    /// assert!(!image2.has_data());
    /// ```
    pub fn has_data(&self) -> bool {
        self.image_data.is_some()
    }
}

/// Actual image data, pixels with values.
///
/// Image providers should implement this trait to allow projects to convert points in space to
/// color values.
pub trait ImageData {
    /// Returns the file path to this image.
    ///
    /// # Examples
    ///
    /// Infratec images implement `ImageData`.
    ///
    /// ```
    /// use riscan_pro::infratec::Image;
    /// use riscan_pro::ImageData;
    /// let path = "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv";
    /// let image = Image::from_path(path).unwrap();
    /// assert_eq!(path, &image.path().to_string_lossy());
    /// ```
    fn path(&self) -> &Path;

    /// Returns the pixel value at the given row and column.
    ///
    /// If the row or column are outside of the image bounds, `ImageData::get` returns `None`.
    ///
    /// # Examples
    ///
    /// Infratec images implement `ImageData`.
    ///
    /// ```
    /// use riscan_pro::infratec::Image;
    /// use riscan_pro::ImageData;
    /// let path = "data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv";
    /// let image = Image::from_path(path).unwrap();
    /// let color = image.get(1., 1.).unwrap();
    /// ```
    fn get(&self, u: f64, v: f64) -> Option<f64>;
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImageData {{ path: {} }}", self.path().to_string_lossy())
    }
}

impl PartialEq for ImageData {
    fn eq(&self, other: &ImageData) -> bool {
        self.path() == other.path()
    }
}

pub fn read_image_data<P: AsRef<Path>>(path: P) -> Result<Option<Box<ImageData>>> {
    let csvfile = path.as_ref().with_extension("csv");
    if fs::metadata(&csvfile).map(|m| m.is_file()).unwrap_or(false) {
        Ok(Some(Box::new(try!(infratec::Image::from_path(csvfile)))))
    } else {
        Ok(None)
    }
}
