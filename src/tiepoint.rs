/// A tie point is used to register scans together or to a global coordinate system.
#[derive(Clone, Debug, Default)]
pub struct Tiepoint {
    /// The name of the tiepoint.
    pub name: String,
    /// The x value of the tiepoint.
    pub x: f64,
    /// The y value of the tiepoint.
    pub y: f64,
    /// The z value of the tiepoint.
    pub z: f64,
    /// The height of the reflector.
    pub height: f64,
}
