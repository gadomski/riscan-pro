/// TLS scan.
#[derive(Debug, PartialEq)]
pub struct Scan {
    name: String,
}

impl Scan {
    /// Creates a new scan.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Scan;
    /// let scan = Scan::new("myscan");
    /// ```
    pub fn new(name: &str) -> Scan {
        Scan { name: name.to_string() }
    }

    /// Returns this scan's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Scan;
    /// let scan = Scan::new("myscan");
    /// assert_eq!("myscan", scan.name());
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}
