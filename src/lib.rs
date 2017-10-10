//! Inspect and do work with RiSCAN Pro projects.
//!
//! [RiSCAN PRO](http://www.riegl.com/products/software-packages/riscan-pro/) is [Terrestrial Laser
//! Scanning (TLS)](https://en.wikipedia.org/wiki/Lidar#Terrestrial_Lidar) software made by
//! [Riegl](http://riegl.com/). This crate provides a library and an executable for inspecting and
//! doing work with RiSCAN Pro projects.
//!
//! **Riegl did not make this software, and cannot provide any support for it.**
//! **Please do not contact Riegl with questions about this software.**
//!
//! # Examples
//!
//! Projects can be opened by providing the project's directory to `Project::from_path`:
//!
//! ```
//! use riscan_pro::Project;
//! let project = Project::from_path("data/project.RiSCAN").unwrap();
//! ```
//!
//! or the `project.rsp` file:
//!
//! ```
//! use riscan_pro::Project;
//! let project = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
//! ```

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_qualifications)]

extern crate alga;
#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate nalgebra;
extern crate regex;
extern crate xmltree;

mod camera;
mod element;
mod point;
mod project;
mod scan_position;
mod utils;

pub use camera::Camera;
pub use point::{Cmcs, Glcs, Prcs, Socs};
pub use project::Project;
pub use scan_position::ScanPosition;

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// The rsp root element is invalid.
    InvalidRspRoot(String),
    /// The matrix does not have an inverse.
    Inverse(nalgebra::Matrix4<f64>),
    /// Wrapper around `std::io::Error`.
    Io(std::io::Error),
    /// A camera setting is missing.
    MissingCameraSetting(String),
    /// The child is missing from the root element.
    MissingChild(String),
    /// There are multiple cameras, which is not supported by this library.
    MultipleCameras,
    /// There is no text in the element.
    NoText(xmltree::Element),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(std::num::ParseFloatError),
    /// Wrapper around `std::num::ParseIntError`.
    ParseInt(std::num::ParseIntError),
    /// Error when parsing a Matrix4 from a string.
    ParseMatrix4(String),
    /// Invalid project path.
    ProjectPath(std::path::PathBuf),
    /// Wrapper around xmltree::ParseError.
    XmltreeParse(xmltree::ParseError),
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<xmltree::ParseError> for Error {
    fn from(err: xmltree::ParseError) -> Error {
        Error::XmltreeParse(err)
    }
}

impl element::FromElement for nalgebra::Projective3<f64> {
    fn from_element(element: &xmltree::Element) -> Result<nalgebra::Projective3<f64>> {
        use element::Extension;
        element.as_str().and_then(|s| utils::projective_from_str(s))
    }
}
