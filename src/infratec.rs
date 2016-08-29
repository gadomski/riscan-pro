//! Infratec makes thermal cameras, which we use in conjunction with scanners.
//!
//! However, the integration of these cameras with the scanners is not complete, so we need to do
//! some outside-of-RiSCAN-Pro processing.

use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read};
use std::path::{Path, PathBuf};

use {Error, Result};
use project::ImageData;

/// Infratec thermal camera image.
#[derive(Debug)]
pub struct Image {
    path: PathBuf,
    header: Header,
    data: Vec<Vec<f64>>,
}

impl Image {
    /// Creates a new image from a path to a csv file.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::infratec::Image;
    /// let image = Image::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv").unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Image> {
        let mut buf = Vec::new();
        try!(File::open(&path).and_then(|mut f| f.read_to_end(&mut buf)));
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
            data: data,
            header: header,
            path: path.as_ref().to_path_buf(),
        })
    }

    /// Returns this image's width.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::infratec::Image;
    /// # let image = Image::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv").unwrap();
    /// let width = image.width();
    /// ```
    pub fn width(&self) -> usize {
        self.header.width
    }

    /// Returns this image's height.
    ///
    /// # Examples
    ///
    /// ```
    /// # use riscan_pro::infratec::Image;
    /// # let image = Image::from_path("data/project.RiSCAN/SCANS/SP01/SCANPOSIMAGES/SP01 - Image001.csv").unwrap();
    /// let height = image.height();
    /// ```
    pub fn height(&self) -> usize {
        self.header.height
    }
}

impl ImageData for Image {
    fn path(&self) -> &Path {
        &self.path
    }

    fn get(&self, u: f64, v: f64) -> Option<f64> {
        // Images are rotated 90 counterclockwise and flipped up-down, which leads to this
        // wonky checking and fetching.
        if u > 0. && v > 0. && u < self.width() as f64 && v < self.height() as f64 {
            Some(self.data[v as usize][u as usize])
        } else {
            None
        }
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
        assert_eq!(3, image.header.version);
        assert_eq!(768, image.height());
        assert_eq!(1024, image.width());
        assert_eq!(768, image.data.len());
        assert!(image.data.iter().all(|v| v.len() == 1024));
        assert_eq!(-38.64, image.data[0][0]);
        assert_eq!(23.84, *image.data.last().unwrap().last().unwrap());
    }
}
