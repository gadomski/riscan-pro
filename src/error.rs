use std::io;
use std::num;
use xml;

/// Our custom error enum.
#[derive(Debug)]
pub enum Error {
    /// Wrapper around `std::io::Error`.
    Io(io::Error),
    /// The provided vector cannot be converted into a matrix.
    Matrix(Vec<f64>),
    /// Wrapper around `std::num::ParseFloatError`.
    ParseFloat(num::ParseFloatError),
    /// An error occured while reading the xml file.
    XmlRead(String),
    /// Wrapper aroudn `xml::reader::Error`.
    XmlReader(xml::reader::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}

impl From<xml::reader::Error> for Error {
    fn from(err: xml::reader::Error) -> Error {
        Error::XmlReader(err)
    }
}
