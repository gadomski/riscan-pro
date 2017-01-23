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
//! # use riscan_pro::Project;
//! let project = Project::from_path("data/project.RiSCAN").unwrap();
//! ```
//!
//! or the `project.rsp` file:
//!
//! ```
//! # use riscan_pro::Project;
//! let project = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
//! ```

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_qualifications)]

extern crate nalgebra;
extern crate xml;

mod error;
mod project;
mod scan;
mod scan_position;

pub use error::Error;
pub use project::Project;
pub use scan::Scan;
pub use scan_position::ScanPosition;

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Type alias for a 4x4 f64 matrix.
pub type Matrix = nalgebra::Matrix4<f64>;

/// Type alias for a 4-element f64 vector.
pub type Vector = nalgebra::Vector4<f64>;
