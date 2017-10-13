use MountCalibration;
use nalgebra::{Point3, Projective3};
use std::marker::PhantomData;
use std::ops::Deref;

/// A three-dimensional point.
#[derive(Clone, Copy, Debug)]
pub struct Point<C: CoordinateReferenceSystem> {
    phantom: PhantomData<C>,
    point: Point3<f64>,
}

/// A marker trait for coordinate reference systems.
pub trait CoordinateReferenceSystem {}

/// The GLobal Coordinate System.
#[derive(Clone, Copy, Debug)]
pub struct Glcs {}

/// The PRoject Coordinate System.
#[derive(Clone, Copy, Debug)]
pub struct Prcs {}

/// The Scanner's Own Coordiate System.
#[derive(Clone, Copy, Debug)]
pub struct Socs {}

/// The CaMera's Coordinate System.
#[derive(Clone, Copy, Debug)]
pub struct Cmcs {}

impl Point<Glcs> {
    /// Returns a point in the global coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// let glcs = Point::glcs(1., 2., 3.);
    /// ```
    pub fn glcs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Glcs> {
        Point {
            phantom: PhantomData,
            point: Point3::new(x.into(), y.into(), z.into()),
        }
    }

    /// Converts this glcs point to prcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Point, Project};
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let prcs = Point::glcs(1., 2., 3.).to_prcs(project.pop);
    /// ```
    pub fn to_prcs(&self, pop: Projective3<f64>) -> Point<Prcs> {
        (pop.inverse() * self.deref()).into()
    }
}

impl Point<Prcs> {
    /// Returns a point in the project's coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// let prcs = Point::prcs(1., 2., 3.);
    /// ```
    pub fn prcs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Prcs> {
        Point {
            phantom: PhantomData,
            point: Point3::new(x.into(), y.into(), z.into()),
        }
    }

    /// Converts this point to the global coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Point, Project};
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let glcs = Point::prcs(1., 2., 3.).to_glcs(project.pop);
    /// ```
    pub fn to_glcs(&self, pop: Projective3<f64>) -> Point<Glcs> {
        (pop * self.deref()).into()
    }

    /// Converts this prcs point to socs.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Point, Project};
    /// let scan_position = Project::from_path("data/project.RiSCAN")
    ///     .unwrap()
    ///     .scan_positions
    ///     .get("SP01")
    ///     .unwrap()
    ///     .clone();
    /// let socs = Point::prcs(1., 2., 3.).to_socs(scan_position.sop);
    /// ```
    pub fn to_socs(&self, sop: Projective3<f64>) -> Point<Socs> {
        (sop.inverse() * self.deref()).into()
    }
}

impl Point<Socs> {
    /// Returns a point in the scanner's own coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// let socs = Point::socs(1., 2., 3.);
    /// ```
    pub fn socs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Socs> {
        Point {
            phantom: PhantomData,
            point: Point3::new(x.into(), y.into(), z.into()),
        }
    }

    /// Converts this socs point to the project coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Point, Project};
    /// let scan_position = Project::from_path("data/project.RiSCAN")
    ///     .unwrap()
    ///     .scan_positions
    ///     .get("SP01")
    ///     .unwrap()
    ///     .clone();
    /// let prcs = Point::socs(1., 2., 3.).to_prcs(scan_position.sop);
    /// ```
    pub fn to_prcs(&self, sop: Projective3<f64>) -> Point<Prcs> {
        (sop * self.deref()).into()
    }

    /// Converts this socs point to cmcs.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Point, Project};
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let image = project.scan_positions
    ///     .get("SP01")
    ///     .unwrap()
    ///     .images
    ///     .get("SP01 - Image001")
    ///     .unwrap();
    /// let mount_calibration = image.mount_calibration(&project).unwrap();
    /// let cmcs = Point::socs(1., 2., 3.).to_cmcs(image.cop, &mount_calibration);
    /// ```
    pub fn to_cmcs(
        &self,
        cop: Projective3<f64>,
        mount_calibration: &MountCalibration,
    ) -> Point<Cmcs> {
        (mount_calibration.deref() * cop.inverse() * self.deref()).into()
    }
}

impl Point<Cmcs> {
    /// Returns a point in the camera's coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// let cmcs = Point::cmcs(1., 2., 3.);
    /// ```
    pub fn cmcs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Cmcs> {
        Point {
            phantom: PhantomData,
            point: Point3::new(x.into(), y.into(), z.into()),
        }
    }

    /// Converts this cmcs point to socs.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Point, Project};
    /// let project = Project::from_path("data/project.RiSCAN").unwrap();
    /// let image = project.scan_positions
    ///     .get("SP01")
    ///     .unwrap()
    ///     .images
    ///     .get("SP01 - Image001")
    ///     .unwrap();
    /// let mount_calibration = image.mount_calibration(&project).unwrap();
    /// let socs = Point::cmcs(1., 2., 3.).to_socs(image.cop, &mount_calibration);
    /// ```
    pub fn to_socs(
        &self,
        cop: Projective3<f64>,
        mount_calibration: &MountCalibration,
    ) -> Point<Socs> {
        (cop * (*mount_calibration).inverse() * self.deref()).into()
    }

    /// Is this point behind the camera?
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// assert!(!Point::cmcs(1., 2., 3.).is_behind_camera());
    /// assert!(Point::cmcs(1., 2., -3.).is_behind_camera());
    /// ```
    pub fn is_behind_camera(&self) -> bool {
        self.z <= 0.
    }

    /// Returns the horizontal tangent of this point.
    ///
    /// The horizontal tangent is the tangent of the angle of the point, as projected to the yz
    /// plane, to the z axis, i.e. `y / z`.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// assert_eq!(2., Point::cmcs(3., 2., 1.).tan_horz());
    /// ```
    pub fn tan_horz(&self) -> f64 {
        self.y / self.z
    }

    /// Returns the vertical tangent of this point.
    ///
    /// The vertical tangent is the tangent of the angle of the point, as projected to the xz
    /// plane, to the z axis, i.e. `x / z`.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::Point;
    /// assert_eq!(3., Point::cmcs(3., 2., 1.).tan_vert());
    /// ```
    pub fn tan_vert(&self) -> f64 {
        self.x / self.z
    }
}

impl<C: CoordinateReferenceSystem> From<Point3<f64>> for Point<C> {
    fn from(point: Point3<f64>) -> Point<C> {
        Point {
            phantom: PhantomData,
            point: point,
        }
    }
}

impl<C: CoordinateReferenceSystem> Deref for Point<C> {
    type Target = Point3<f64>;
    fn deref(&self) -> &Point3<f64> {
        &self.point
    }
}

impl<C: CoordinateReferenceSystem> PartialEq<Point<C>> for Point<C> {
    fn eq(&self, other: &Point<C>) -> bool {
        self.deref().eq(other)
    }
}

impl CoordinateReferenceSystem for Glcs {}
impl CoordinateReferenceSystem for Prcs {}
impl CoordinateReferenceSystem for Socs {}
impl CoordinateReferenceSystem for Cmcs {}

#[cfg(test)]
mod tests {
    use super::*;
    use Project;

    #[test]
    fn cmcs_is_behind_camera() {
        assert!(!Point::cmcs(1., 1., 1.).is_behind_camera());
        assert!(Point::cmcs(1., 1., -1.).is_behind_camera());
    }

    #[test]
    fn roundtrip() {
        let glcs = Point::glcs(1., 2., 3.);
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_positions.get("SP01").unwrap();
        let image = scan_position.images.get("SP01 - Image001").unwrap();
        let mount_calibration = image.mount_calibration(&project).unwrap();

        let prcs = glcs.to_prcs(project.pop);
        let socs = prcs.to_socs(scan_position.sop);
        let cmcs = socs.to_cmcs(image.cop, &mount_calibration);
        let socs2 = cmcs.to_socs(image.cop, &mount_calibration);
        assert_relative_eq!(socs.deref(), socs2.deref(), epsilon = 1e-6);
        let prcs2 = socs2.to_prcs(scan_position.sop);
        assert_relative_eq!(prcs.deref(), prcs2.deref(), epsilon = 1e-6);
        let glcs2 = prcs2.to_glcs(project.pop);
        assert_relative_eq!(glcs.deref(), glcs2.deref(), epsilon = 1e-6);
    }
}
