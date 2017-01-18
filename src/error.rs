use std::io;
use std::num;
use xml;
use xml::reader::XmlEvent;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Matrix(Vec<f64>),
    MissingAttribute(String),
    ParseFloat(num::ParseFloatError),
    UnexpectedXmlEvent(XmlEvent),
    XmlRead(String),
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
