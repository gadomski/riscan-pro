extern crate nalgebra;
extern crate xml;

mod error;
mod project;
mod scan_position;

pub use error::Error;
pub use project::Project;
pub use scan_position::ScanPosition;

pub type Result<T> = std::result::Result<T, Error>;
pub type Matrix = nalgebra::Matrix4<f64>;
pub type Vector = nalgebra::Vector4<f64>;
