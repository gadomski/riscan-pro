use nalgebra::Projective3;
use std::ops::Deref;

/// A camera mount calibration.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct MountCalibration {
    /// The calibration matrix.
    pub matrix: Projective3<f64>,
    /// The name of the calibration matrix.
    pub name: String,
}

impl Deref for MountCalibration {
    type Target = Projective3<f64>;
    fn deref(&self) -> &Projective3<f64> {
        &self.matrix
    }
}
