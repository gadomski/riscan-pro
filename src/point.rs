//! Three-dimensional points.
//!
//! Includes a type system to enforce coordinate systems.

use Camera;
use alga::general::SubsetOf;
use nalgebra::{Point3, Projective3};
use std::marker::PhantomData;

macro_rules! coordinate_system  {
    ($name:ident) => {
        #[allow(missing_docs)]
        #[derive(Clone, Copy, Debug)]
        pub struct $name {}

        impl From<Point3<f64>> for Point<$name> {
            fn from(point: Point3<f64>) -> Point<$name> {
                Point {
                    point: point,
                    crs: PhantomData,
                }
            }
        }

        impl PartialEq<Point3<f64>> for Point<$name> {
            fn eq(&self, other: &Point3<f64>) -> bool {
                self.point.eq(other)
            }
        }

        impl PartialEq<Point<$name>> for Point<$name> {
            fn eq(&self, other: &Point<$name>) -> bool {
                self.point.eq(&other.point)
            }
        }
    }
}

/// A point in a coordinate reference system.
#[derive(Debug, PartialEq)]
pub struct Point<C> {
    point: Point3<f64>,
    crs: PhantomData<C>,
}

/// The GLobal Coordinate System.
pub fn glcs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Glcs> {
    Point::new(x, y, z)
}
coordinate_system!(Glcs);

/// The PRoject's Coordinate System.
pub fn prcs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Prcs> {
    Point::new(x, y, z)
}
coordinate_system!(Prcs);

/// The Scanner's Own Coordinate System.
pub fn socs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Socs> {
    Point::new(x, y, z)
}
coordinate_system!(Socs);

/// The CaMera's Coordinate System.
pub fn cmcs<T: Into<f64>>(x: T, y: T, z: T) -> Point<Cmcs> {
    Point::new(x, y, z)
}
coordinate_system!(Cmcs);

impl<C> Point<C> {
    fn new<T>(x: T, y: T, z: T) -> Point<C>
        where T: Into<f64>
    {
        Point {
            point: Point3::new(x.into(), y.into(), z.into()),
            crs: PhantomData,
        }
    }

    /// Returns a reference to this point as a bare `Point3`.
    ///
    /// Use this to compare raw values without coordinate systems.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::point;
    /// let socs = point::socs(1., 2., 3.);
    /// let glcs = point::glcs(1., 2., 3.);
    /// assert_eq!(socs.as_point3(), glcs.as_point3());
    /// ```
    pub fn as_point3(&self) -> &Point3<f64> {
        &self.point
    }
}

impl Point<Glcs> {
    /// Converts a point from the global coordinate system into the project's coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate nalgebra;
    /// # extern crate riscan_pro;
    /// # fn main() {
    /// use riscan_pro::point;
    /// use nalgebra::Projective3;
    /// let glcs = point::glcs(1., 2., 3.);
    /// let eye = Projective3::<f64>::identity();
    /// let prcs = glcs.to_prcs(eye);
    /// assert_eq!(glcs.as_point3(), prcs.as_point3());
    /// # }
    /// ```
    pub fn to_prcs<T>(&self, pop: T) -> Point<Glcs>
        where T: SubsetOf<Projective3<f64>>
    {
        (pop.to_superset().inverse() * self.point).into()
    }
}

impl Point<Prcs> {
    /// Converts a point from the project's coordinate system into the global coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate nalgebra;
    /// # extern crate riscan_pro;
    /// # fn main() {
    /// use riscan_pro::point;
    /// use nalgebra::Projective3;
    /// let prcs = point::prcs(1., 2., 3.);
    /// let eye = Projective3::<f64>::identity();
    /// let glcs = prcs.to_glcs(eye);
    /// assert_eq!(glcs.as_point3(), prcs.as_point3());
    /// # }
    /// ```
    pub fn to_glcs<T>(&self, pop: T) -> Point<Glcs>
        where T: SubsetOf<Projective3<f64>>
    {
        (pop.to_superset() * self.point).into()
    }

    /// Converts a point from the project's coordinate system into the camera's coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate nalgebra;
    /// # extern crate riscan_pro;
    /// # fn main() {
    /// use riscan_pro::point;
    /// use nalgebra::Projective3;
    /// let prcs = point::prcs(1., 2., 3.);
    /// let eye = Projective3::<f64>::identity();
    /// let cmcs = prcs.to_cmcs(eye, eye, eye);
    /// // assert_eq!(prcs, cmcs); <-- compile error
    /// assert_eq!(prcs.as_point3(), cmcs.as_point3());
    /// }
    /// ```
    pub fn to_cmcs<A, B, C>(&self, sop: A, mounting_matrix: B, cop: C) -> Point<Cmcs>
        where A: SubsetOf<Projective3<f64>>,
              B: SubsetOf<Projective3<f64>>,
              C: SubsetOf<Projective3<f64>>
    {
        use alga::linear::Transformation;
        (mounting_matrix.to_superset() * cop.to_superset().inverse() * sop.to_superset().inverse())
            .transform_point(&self.point)
            .into()
    }
}

impl Point<Cmcs> {
    /// Convert camera's coordinates to image coordinate space.
    ///
    /// Returns the distorted coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Camera, point};
    /// let camera = Camera::from_path("data/camera.cam").unwrap();
    /// let point = point::cmcs(1., 2., 3.);
    /// let (u, v) = point.to_ics(&camera);
    /// ```
    pub fn to_ics(&self, camera: &Camera) -> (f64, f64) {
        use nalgebra::Matrix3;

        let a = Matrix3::new(camera.fx,
                             0.,
                             camera.cx,
                             0.,
                             camera.fy,
                             camera.cy,
                             0.,
                             0.,
                             1.);
        let ud_prime = a * self.point;
        let u = ud_prime[0] / ud_prime[2];
        let v = ud_prime[1] / ud_prime[2];
        let x = (u - camera.cx) / camera.fx;
        let y = (v - camera.cy) / camera.fy;
        let r = (x.powi(2) + y.powi(2))
            .sqrt()
            .atan()
            .powi(2)
            .sqrt();
        let r_term = camera.k1 * r.powi(2) + camera.k2 * r.powi(4) + camera.k3 * r.powi(6) +
                     camera.k4 * r.powi(8);
        let u = u + x * camera.fx * r_term + 2. * camera.fx * x * y * camera.p1 +
                camera.p2 * camera.fx * (r.powi(2) + 2. * x.powi(2));
        let v = v + y * camera.fy * r_term + 2. * camera.fy * x * y * camera.p2 +
                camera.p1 * camera.fy * (r.powi(2) + 2. * y.powi(2));
        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Projective3, Rotation3};
    use std::f64::consts::PI;

    #[test]
    fn prcs_to_cmcs_identity() {
        let prcs: Point<Prcs> = Point::new(1., 2., 3.);
        let cmcs = prcs.to_cmcs(Projective3::<f64>::identity(),
                                Projective3::<f64>::identity(),
                                Projective3::<f64>::identity());
        assert_eq!(cmcs, *prcs.as_point3());
    }

    #[test]
    fn prcs_to_cmcs_mounting_matrix() {
        let prcs: Point<Prcs> = Point::new(1., 2., 3.);
        let cmcs = prcs.to_cmcs(Projective3::<f64>::identity(),
                                Rotation3::<f64>::from_euler_angles(0., 0., PI / 2.),
                                Projective3::<f64>::identity());
        assert_relative_eq!(Point3::new(-2., 1., 3.), cmcs.as_point3());
    }

    #[test]
    fn prcs_to_cmcs_cop() {
        let prcs: Point<Prcs> = Point::new(1., 2., 3.);
        let cmcs = prcs.to_cmcs(Projective3::<f64>::identity(),
                                Projective3::<f64>::identity(),
                                Rotation3::<f64>::from_euler_angles(0., 0., PI / 2.));
        assert_relative_eq!(Point3::new(2., -1., 3.), cmcs.as_point3());
    }

    #[test]
    fn prcs_to_cmcs_sop() {
        let prcs: Point<Prcs> = Point::new(1., 2., 3.);
        let cmcs = prcs.to_cmcs(Rotation3::<f64>::from_euler_angles(0., 0., PI / 2.),
                                Projective3::<f64>::identity(),
                                Projective3::<f64>::identity());
        assert_relative_eq!(Point3::new(2., -1., 3.), cmcs.as_point3());
    }

    #[test]
    fn cmcs_to_ics() {
        let cmcs: Point<Cmcs> = Point::new(1., 2., 3.);
        let camera = Camera::from_path("data/camera.cam").unwrap();
        let (u, v) = cmcs.to_ics(&camera);
        assert_relative_eq!(777.5760, u, epsilon = 1e-4);
        assert_relative_eq!(896.7450, v, epsilon = 1e-4);
    }
}
