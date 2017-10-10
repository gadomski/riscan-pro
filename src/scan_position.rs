use Result;
use element::FromElement;
use std::path::Path;
use xmltree::Element;

/// A scan position.
#[derive(Debug, PartialEq)]
pub struct ScanPosition {
    singlescans: Vec<Scan>,
}

#[derive(Debug, PartialEq)]
struct Scan {
    file: String,
}

impl ScanPosition {
    /// Returns true if this scan position includes the provided path.
    pub fn has_path<P: AsRef<Path>>(&self, path: P) -> bool {
        self.singlescans.iter().any(|singlescan| {
            path.as_ref()
                .file_name()
                .map(|file_name| file_name.to_string_lossy() == singlescan.file)
                .unwrap_or(false)
        })
    }
}

impl FromElement for ScanPosition {
    fn from_element(element: &Element) -> Result<ScanPosition> {
        use element::Extension;
        let singlescans = element.xpath("singlescans")?
            .children
            .iter()
            .map(|scan| scan.convert())
            .collect::<Result<Vec<_>>>()?;
        Ok(ScanPosition { singlescans: singlescans })
    }
}

impl FromElement for Scan {
    fn from_element(element: &Element) -> Result<Scan> {
        use element::Extension;
        Ok(Scan {
               file: element.xpath("file")?
                   .as_str()?
                   .to_string(),
           })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sp01() -> ScanPosition {
        use Project;
        let project = Project::from_path("data/project.RiSCAN").unwrap();
        project.scan_position("SP01").unwrap().unwrap()
    }

    #[test]
    fn has_path() {
        let scan_position = sp01();
        assert!(scan_position.has_path("151120_150227.rxp"));
        assert!(!scan_position.has_path("151120_150227.rxz"));
    }
}
