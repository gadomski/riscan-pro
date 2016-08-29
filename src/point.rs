use nalgebra::Vector4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T, N>
    where N: Copy
{
    pub crs: T,
    pub x: N,
    pub y: N,
    pub z: N,
}

impl<T> From<Point<T, f64>> for Vector4<f64> {
    fn from(point: Point<T, f64>) -> Vector4<f64> {
        Vector4::new(point.x, point.y, point.z, 1.)
    }
}

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
        let vector = Vector4::from(point);
        assert_eq!(Vector4::new(1., 2., 3., 1.), vector);
    }
}
