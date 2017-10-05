use {Point3, Projective3};

/// A scan position image.
#[derive(Clone, Copy, Debug)]
pub struct Image {
    cop: Projective3,
    mounting_matrix: Projective3,
    sop: Projective3,
}

impl Image {
    /// Converts a point in the project's coordinate system to the camera's coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Image, Point3};
    /// let image = Image::default();
    /// let input = Point3::new(1., 2., 3.);
    /// let output = image.prcs_to_cmcs(input);
    /// assert_eq!(output, input);
    /// ```
    pub fn prcs_to_cmcs(&self, point: Point3) -> Point3 {
        use alga::linear::Transformation;

        (self.mounting_matrix * self.cop.inverse() * self.sop.inverse()).transform_point(&point)
    }
}

impl Default for Image {
    fn default() -> Image {
        Image {
            cop: Projective3::identity(),
            mounting_matrix: Projective3::identity(),
            sop: Projective3::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alga::general::SubsetOf;
    use nalgebra::Rotation3;
    use std::f64::consts::PI;

    #[test]
    fn prcs_to_cmcs_identity() {
        let image = Image::default();
        let input = Point3::new(1., 2., 3.);
        let output = image.prcs_to_cmcs(input);
        assert_eq!(output, input);
    }

    #[test]
    fn prcs_to_cmcs_mounting_matrix() {
        let mut image = Image::default();
        image.mounting_matrix = Rotation3::from_euler_angles(0., 0., PI / 2.).to_superset();
        let input = Point3::new(1., 2., 3.);
        let output = image.prcs_to_cmcs(input);
        assert_relative_eq!(Point3::new(-2., 1., 3.), output);
    }

    #[test]
    fn prcs_to_cmcs_cop() {
        let mut image = Image::default();
        image.cop = Rotation3::from_euler_angles(0., 0., PI / 2.).to_superset();
        let input = Point3::new(1., 2., 3.);
        let output = image.prcs_to_cmcs(input);
        assert_relative_eq!(Point3::new(2., -1., 3.), output);
    }

    #[test]
    fn prcs_to_cmcs_sop() {
        let mut image = Image::default();
        image.sop = Rotation3::from_euler_angles(0., 0., PI / 2.).to_superset();
        let input = Point3::new(1., 2., 3.);
        let output = image.prcs_to_cmcs(input);
        assert_relative_eq!(Point3::new(2., -1., 3.), output);
    }
}
