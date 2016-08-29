use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Read};
use std::path::Path;

use {Error, Result};
use project::ImageData;

pub struct Image {
    pub version: u8,
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<f32>>,
}

impl Image {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Image> {
        let mut buf = Vec::new();
        try!(File::open(path).and_then(|mut f| f.read_to_end(&mut buf)));
        // We need to read everything in, then convert w/ utf8 lossy, because the degree sign
        // doesn't seem to be valid utf8.
        let mut iter = BufReader::new(Cursor::new(String::from_utf8_lossy(&buf[..]).into_owned()))
            .lines();
        let mut header_map: HashMap<String, String> = HashMap::new();
        {
            let mut next_line = || {
                iter.next()
                    .map(|r| r.map_err(Error::from))
                    .unwrap_or(Err(Error::ReadInfratec("Unexpected end of file".to_string())))
            };
            let header = try!(next_line());
            if header != "[Settings]" {
                return Err(Error::ReadInfratec(format!("Invalid header: {}", header)));
            }
            loop {
                let line = try!(next_line());
                if line == "[Data]" {
                    break;
                } else if line.is_empty() {
                    continue;
                } else {
                    let errmsg = format!("Invalid header line: {}", line);
                    let mut iter = line.split('=');
                    {
                        let mut next_word = || {
                            iter.next()
                                .map(|s| s.to_string())
                                .ok_or(Error::ReadInfratec(errmsg.clone()))
                        };
                        header_map.insert(try!(next_word()), try!(next_word()));
                    }
                    if iter.next().is_some() {
                        return Err(Error::ReadInfratec(errmsg));
                    }
                }
            }
        }
        let data: Vec<Vec<f32>> = try!(iter.map(|r| {
                r.map_err(Error::from)
                    .and_then(|s| {
                        s.split(';').map(|s| s.parse::<f32>().map_err(Error::from)).collect()
                    })
            })
            .collect());
        let get_from_header_map = |name| {
            header_map.get(name).ok_or(Error::ReadInfratec(format!("Missing header key: {}", name)))
        };
        Ok(Image {
            version: try!(get_from_header_map("Version")
                .and_then(|s| s.parse::<u8>().map_err(Error::from))),
            width: try!(get_from_header_map("ImageWidth")
                .and_then(|s| s.parse::<usize>().map_err(Error::from))),
            height: try!(get_from_header_map("ImageHeight")
                .and_then(|s| s.parse::<usize>().map_err(Error::from))),
            data: data,
        })
    }
}

impl ImageData for Image {
    fn get(&self, u: f64, v: f64) -> Option<f64> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_from_path() {
        let image = Image::from_path("data/SP01 - Image001.csv").unwrap();
        assert_eq!(3, image.version);
        assert_eq!(768, image.height);
        assert_eq!(1024, image.width);
        assert_eq!(768, image.data.len());
        assert!(image.data.iter().all(|v| v.len() == 1024));
        assert_eq!(-38.64, image.data[0][0]);
        assert_eq!(23.84, *image.data.last().unwrap().last().unwrap());
    }
}
