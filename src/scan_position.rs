use {Matrix, Vector};
use nalgebra::Eye;

/// A scan position.
#[derive(Clone, Debug)]
pub struct ScanPosition {
    name: String,
    pop: Matrix,
    sop: Matrix,
}

impl ScanPosition {
    /// Creates a new scan position.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// let scan_position = ScanPosition::new();
    /// ```
    pub fn new() -> ScanPosition {
        ScanPosition {
            name: String::new(),
            pop: Matrix::new_identity(4),
            sop: Matrix::new_identity(4),
        }
    }

    /// Returns this scan position's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// let mut scan_position = ScanPosition::new();
    /// scan_position.set_name("ScanPos001");
    /// assert_eq!("ScanPos001", scan_position.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets this scan position's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let mut scan_position = ScanPosition::new();
    /// scan_position.set_name("New name");
    /// ```
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /// Returns this scan position's SOP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let scan_position = ScanPosition::new();
    /// let sop = scan_position.sop();
    /// ```
    pub fn sop(&self) -> Matrix {
        self.sop
    }

    /// Sets this scan position's SOP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{ScanPosition, Matrix};
    /// # let mut scan_position = ScanPosition::new();
    /// scan_position.set_sop(Matrix::new(1., 0., 0., 1.,
    ///                                   0., 1., 0., 2.,
    ///                                   0., 0., 1., 3.,
    ///                                   0., 0., 0., 1.));
    /// ```
    pub fn set_sop(&mut self, sop: Matrix) {
        self.sop = sop;
    }

    /// Returns this scan position's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let scan_position = ScanPosition::new();
    /// let pop = scan_position.pop();
    /// ```
    pub fn pop(&self) -> Matrix {
        self.pop
    }

    /// Sets this scan position's POP.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::{ScanPosition, Matrix};
    /// # let mut scan_position = ScanPosition::new();
    /// scan_position.set_pop(Matrix::new(1., 0., 0., 1.,
    ///                                   0., 1., 0., 2.,
    ///                                   0., 0., 1., 3.,
    ///                                   0., 0., 0., 1.));
    /// ```
    pub fn set_pop(&mut self, pop: Matrix) {
        self.pop = pop;
    }

    /// Converts SOCS coordinates to GLCS coordinates.
    ///
    /// Convert (0., 0., 0.) to get the scanner's origin in GLCS.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::ScanPosition;
    /// # let scan_position = ScanPosition::new();
    /// let (x, y, z) = scan_position.socs_to_glcs((1., 2., 3.));
    /// ```
    pub fn socs_to_glcs(&self, (x, y, z): (f64, f64, f64)) -> (f64, f64, f64) {
        let glcs = self.pop * self.sop * Vector::new(x, y, z, 1.);
        (glcs.x, glcs.y, glcs.z)
    }
}

#[cfg(test)]
mod tests {
    use Project;

    #[test]
    fn scan_position_glcs() {
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        let scan_position = project.scan_position("SP01").unwrap();
        let (x, y, z) = scan_position.socs_to_glcs((1., 2., 3.));
        assert!((-515633.63 - x).abs() < 1e-2);
        assert!((-5519674.02 - y).abs() < 1e-2);
        assert!((3143445.58 - z).abs() < 1e-2);
    }
}
