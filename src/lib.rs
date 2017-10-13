//! Crack open RiSCAN Pro xml files.
//!
//! [RiSCAN Pro](http://www.riegl.com/products/software-packages/riscan-pro/) is software developed
//! by [Riegl](http://riegl.com/) for [terrestrial LiDAR
//! scanning](https://en.wikipedia.org/wiki/Lidar#Terrestrial_lidar). RiSCAN Pro stores most
//! project metadata, e.g. calibration and transformation matrices, in a xml file inside of the
//! RiSCAN Pro project directory. This is a Rust library for reading these xml files and extracting
//! the good bits.
//!
//! **This project is not created by Riegl and no support from them is provided or implied.
//! Please do not contact Riegl about this software.**
//!
//! This library is not complete, as there's lots of project components that aren't supported. This
//! was developed for a specific purpose (colorizing points and transforming them) and so far
//! hasn't been developed much beyond that.
//!
//! # Examples
//!
//! Projects can be opened from the directory or the rsp file themselves:
//!
//! ```
//! use riscan_pro::Project;
//! let project1 = Project::from_path("data/project.RiSCAN").unwrap();
//! let project2 = Project::from_path("data/project.RiSCAN/project.rsp").unwrap();
//! assert_eq!(project1, project2);
//! ```
//!
//! Everything available to you is a public attribute of the project. For example, to transform a
//! point in the project's coordinate system (PRCS) to the global coordinate system (GLCS), use the
//! `pop` attribute of the project. Points are typed so they can't be compared directly, but they
//! can be dereferenced into their underlying `nalgebra::Point3<f64>`.
//!
//! ```
//! use riscan_pro::{Point, Project};
//! let project = Project::from_path("data/project.RiSCAN").unwrap();
//! let prcs = Point::prcs(1., 2., 3.);
//! let glcs = prcs.to_glcs(project.pop);
//! // assert!(prcs != glcs) <-- compile error
//! assert!(*prcs != *glcs);
//! ```

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]
#![recursion_limit="128"]

#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate nalgebra;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate serde_derive;
extern crate xmltree;

mod camera_calibration;
pub mod element;
mod mount_calibration;
mod point;
mod project;
pub mod scan_position;
mod utils;

pub use camera_calibration::CameraCalibration;
pub use mount_calibration::MountCalibration;
pub use point::{Cmcs, Glcs, Point, Prcs, Socs};
pub use project::Project;
pub use scan_position::ScanPosition;

quick_error! {
/// Our custom error enum.
    #[derive(Debug)]
    pub enum Error {
        /// The camera calibration version is unsupported.
        CameraCalibrationVersion(version: String) {
            description("invalid camera calibration version")
            display("This camera calibration version is not supported: {}", version)
        }
        /// Given a path and a project, could not find an image.
        ImageFromPath(path: std::path::PathBuf) {
            description("could not create image from project and path")
            display("Could not create image from path: {}", path.display())
        }
        /// Wrapper around `std::io::Error`.
        Io(err: std::io::Error) {
            description(err.description())
            display("IO error: {}", err)
            from()
            cause(err)
        }
        /// There is no camera calibration with the given name.
        MissingCameraCalibration(name: String) {
            description("the camera calibration does not exist")
            display("The camera calibration does not exist: {}", name)
        }
        /// A requested xml element child does not exist.
        MissingChild(parent: String, child: String) {
            description("the child element does not exist")
            display("The element {} is not a child of {}", parent, child)
        }
        /// There is no mount calibration with the given name.
        MissingMountCalibration(name: String) {
            description("the mount calibration does not exist")
            display("The mount calibration does not exist: {}", name)
        }
        /// There is no noderef attribute on an element.
        MissingNoderef(element: xmltree::Element) {
            description("the element does not have a noderef attribute")
            display("The element named {} does not have a noderef attribute", element.name)
        }
        /// The element does not have any text, when it was required.
        NoElementText(element: xmltree::Element) {
            description("the element does not have text")
            display("The element named {} does not have text", element.name)
        }
        /// Wrapper around `std::num::ParseFloatError`.
        ParseFloat(err: std::num::ParseFloatError) {
            description(err.description())
            display("Parse float error: {}", err)
            from()
            cause(err)
        }
        /// Wrapper around `std::num::ParseFloatError`.
        ParseInt(err: std::num::ParseIntError) {
            description(err.description())
            display("Parse int error: {}", err)
            from()
            cause(err)
        }
        /// Unable to parse text as projective3 matrix.
        ParseProjective3(text: String) {
            description("cannot parse text as Projective3")
            display("Cannot parse text as Projective3: {}", text)
        }
        /// The path is not a valid project path.
        ///
        /// Valid project paths either end in .rsp or .RiSCAN.
        ProjectPath(path: std::path::PathBuf) {
            description("invalid project path")
            display("Invalid project path: {}", path.display())
        }
        /// The scan position could not be found from the provided path.
        ScanPositionFromPath(path: std::path::PathBuf) {
            description("cound not find scan position in project from path")
            display("Path {} does not refer to a scan position", path.display())
        }
        /// Wrapper around `xmltree::ParseError`.
        XmltreeParse(err: xmltree::ParseError) {
            description(err.description())
            display("Xmltree parse error: {}", err)
            from()
            cause(err)
        }
    }
}

/// Our custom result type.
pub type Result<T> = std::result::Result<T, Error>;
