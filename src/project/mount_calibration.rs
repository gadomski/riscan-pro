use nalgebra::Matrix4;

/// A mounting calibration for a camera or something else.
#[derive(Clone, Debug, PartialEq)]
pub struct MountCalibration {
    matrix: Matrix4<f64>,
    name: String,
}

impl MountCalibration {
    /// Creates a new mount calibration.
    pub fn new(name: &str, matrix: Matrix4<f64>) -> MountCalibration {
        MountCalibration {
            matrix: matrix,
            name: name.to_string(),
        }
    }

    /// Returns this mount calibration's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns this mount calibration's matrix.
    pub fn matrix(&self) -> Matrix4<f64> {
        self.matrix
    }
}
