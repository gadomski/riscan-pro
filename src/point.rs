use nalgebra::Point3;
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

/// The Scanner's Own Coordiate System.
#[derive(Clone, Copy, Debug)]
pub struct Socs {}

/// The CaMera's Coordinate System.
#[derive(Clone, Copy, Debug)]
pub struct Cmcs {}

impl Point<Glcs> {
    /// Returns a point in the scanner's own coordinate system.
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

impl CoordinateReferenceSystem for Glcs {}
impl CoordinateReferenceSystem for Socs {}
impl CoordinateReferenceSystem for Cmcs {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmcs_is_behind_camera() {
        assert!(!Point::cmcs(1., 1., 1.).is_behind_camera());
        assert!(Point::cmcs(1., 1., -1.).is_behind_camera());
    }
}
