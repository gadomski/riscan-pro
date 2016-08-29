use nalgebra::Vector4;

/// A three-dimensional point with a coordinate reference system.
///
/// # Examples
///
/// ```
/// use riscan_pro::{Point, PRCS};
/// let point = Point {
///     crs: PRCS, // Project's Reference Coordinate System
///     x: 1f64,
///     y: 2.,
///     z: 3.,
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T, N>
    where N: Copy
{
    /// The coordinate reference system.
    pub crs: T,
    /// X coordinate.
    pub x: N,
    /// Y coordinate.
    pub y: N,
    /// Z coordinate.
    pub z: N,
}

impl<T> From<Point<T, f64>> for Vector4<f64> {
    fn from(point: Point<T, f64>) -> Vector4<f64> {
        Vector4::new(point.x, point.y, point.z, 1.)
    }
}

/// Project's reference coordinate system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRCS;

#[cfg(test)]
mod tests {
    use super::*;

    use nalgebra::Vector4;

    #[test]
    fn vector4_from_point() {
        let point = Point {
            crs: PRCS,
            x: 1f64,
            y: 2.,
            z: 3.,
        };
        assert_eq!(Vector4::new(1., 2., 3., 1.), point.into());
    }
}
