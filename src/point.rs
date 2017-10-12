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
