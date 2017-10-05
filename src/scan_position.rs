use Result;
use element::FromElement;
use xmltree::Element;

/// A scan position.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScanPosition {}

impl FromElement for ScanPosition {
    fn from_element(_: &Element) -> Result<ScanPosition> {
        Ok(ScanPosition {})
    }
}
