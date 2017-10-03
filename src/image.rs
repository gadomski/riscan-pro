use Vector3;

/// A scan position image.
#[derive(Clone, Copy, Debug, Default)]
pub struct Image {}

impl Image {
    /// Converts a point in the project's coordinate system to the camera's coordinate system.
    ///
    /// # Examples
    ///
    /// ```
    /// use riscan_pro::{Image, Vector3};
    /// let image = Image::default();
    /// let input = Vector3::new(1., 2., 3.);
    /// let output = image.prcs_to_cmcs(input);
    /// assert_eq!(output, input);
    /// ```
    pub fn prcs_to_cmcs(&self, point: Vector3) -> Vector3 {
        point
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() {
        let image = Image::default();
        let input = Vector3::new(1., 2., 3.);
        let output = image.prcs_to_cmcs(input);
        assert_eq!(output, input);
    }
}
