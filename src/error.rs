use std::io;
use std::num;

use nalgebra::Matrix4;
use xmltree;

/// Crate-specific error type.
///
/// TODO implement `std::error::Error`.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// Project XML file is missing a required element.
    MissingElement(String),
    /// Project image does not have any associated image data.
    MissingImageData(String, String),
    /// The matrix is not invertable.
    MissingInverse(Matrix4<f64>),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(num::ParseFloatError),
    /// Wrapper around `std::num::ParseIntError`.
    ParseInt(num::ParseIntError),
    /// Unable to parse a matrix from the string.
    ParseMatrix(String),
    /// Error while reading an infratec image.
    ReadInfratec(String),
    /// The camera calibration is not supported by this crate.
    ///
    /// It might be a valid calibration, but we don't know how to handle it (yet).
    UnsupportedCameraCalibration(String),
    /// The OpenCv version.
    ///
    /// It might be a valid version, but we don't know how to handle it (yet).
    UnsupportedOpenCvVersion(u8),
    /// Wrapper around `xmltree::ParseError`.
    XmltreeParse(xmltree::ParseError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<xmltree::ParseError> for Error {
    fn from(err: xmltree::ParseError) -> Error {
        Error::XmltreeParse(err)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}
