use std::io;

use nalgebra::Matrix4;
use xmltree;

#[derive(Debug)]
pub enum Error {
    InvalidXml(String),
    Io(io::Error),
    MissingElement(String),
    MissingInverse(Matrix4<f64>),
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
