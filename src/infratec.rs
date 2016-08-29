use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read};
use std::path::Path;

use {Error, Result};
use project::ImageData;

pub struct Image {
    header: Header,
    data: Vec<Vec<f64>>,
}

impl Image {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Image> {
        let mut buf = Vec::new();
        try!(File::open(path).and_then(|mut f| f.read_to_end(&mut buf)));
        // We need to read everything in, then convert w/ utf8 lossy, because the degree sign
        // doesn't seem to be valid utf8.
        let mut reader = BufReader::new(Cursor::new(String::from_utf8_lossy(&buf[..])
            .into_owned()));
        let header = try!(Header::new(&mut reader));
        let data: Vec<Vec<f64>> = try!(reader.lines()
            .map(|r| {
                r.map_err(Error::from)
                    .and_then(|s| {
                        s.split(';').map(|s| s.parse::<f64>().map_err(Error::from)).collect()
                    })
            })
            .collect());
        Ok(Image {
            header: header,
            data: data,
        })
    }

    pub fn version(&self) -> u8 {
        self.header.version
    }

    pub fn width(&self) -> usize {
        self.header.width
    }

    pub fn height(&self) -> usize {
        self.header.height
    }

    pub fn data(&self) -> &Vec<Vec<f64>> {
        &self.data
    }
}

impl ImageData for Image {
    fn get(&self, u: f64, v: f64) -> Option<f64> {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
struct Header {
    version: u8,
    width: usize,
    height: usize,
}

impl Header {
    fn new<R: BufRead>(reader: &mut R) -> Result<Header> {
        let mut lines = reader.lines();
        let mut next_line = || {
            lines.next()
                .map(|r| r.map_err(Error::from))
                .unwrap_or(Err(Error::ReadInfratec("Unexpected EOF".to_string())))
        };
        let first_line = try!(next_line());
        if first_line != "[Settings]" {
            return Err(Error::ReadInfratec(format!("Invalid first line: {}", first_line)));
        }
        let mut header = Default::default();
        loop {
            let line = try!(next_line());
            if line == "[Data]" {
                return Ok(header);
            } else if line.is_empty() {
                continue;
            }
            let words = line.split('=').collect::<Vec<_>>();
            if words.len() != 2 {
                return Err(Error::ReadInfratec(format!("Invalid header line: {}", line)));
            }
            match words[0] {
                "ImageHeight" => header.height = try!(words[1].parse()),
                "ImageWidth" => header.width = try!(words[1].parse()),
                "Version" => header.version = try!(words[1].parse()),
                _ => {}
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_from_path() {
        let image =
            Image::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv")
                .unwrap();
        assert_eq!(3, image.version());
        assert_eq!(768, image.height());
        assert_eq!(1024, image.width());
        assert_eq!(768, image.data().len());
        assert!(image.data.iter().all(|v| v.len() == 1024));
        assert_eq!(-38.64, image.data()[0][0]);
        assert_eq!(23.84, *image.data().last().unwrap().last().unwrap());
    }
}
