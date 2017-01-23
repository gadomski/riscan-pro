/// A single scan.
#[derive(Clone, Debug)]
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
    /// let scan = Scan::new();
    /// ```
    pub fn new() -> Scan {
        Scan { name: String::new() }
    }

    /// Returns a reference to this scan's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Scan;
    /// let mut scan = Scan::new();
    /// scan.set_name("Beer o' clock");
    /// assert_eq!("Beer o' clock", scan.name());
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets this scan's name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::Scan;
    /// let mut scan = Scan::new();
    /// scan.set_name("Beer o' clock");
    /// ```
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
