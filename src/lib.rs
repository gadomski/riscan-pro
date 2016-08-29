extern crate nalgebra;
extern crate xmltree;

mod error;
mod point;
mod project;

pub use error::Error;
pub use project::Project;

pub type Result<T> = std::result::Result<T, Error>;
