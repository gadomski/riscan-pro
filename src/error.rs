use std::io;
use std::num;

use nalgebra::Matrix4;
use xmltree;

#[derive(Debug)]
pub enum Error {
    InvalidXml(String),
    Io(io::Error),
    MissingElement(String),
    MissingInverse(Matrix4<f64>),
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError),
    ParseMatrix(String),
    UnsupportedCameraCalibration(String),
    UnsupportedOpenCvVersion(u8),
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
