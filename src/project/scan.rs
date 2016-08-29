use xmltree::Element;

use Result;
use project::traits::GetDescendant;

#[derive(Debug, PartialEq)]
pub struct Scan {
    name: String,
}

impl Scan {
    pub fn from_element(element: &Element) -> Result<Scan> {
        Ok(Scan { name: try!(element.get_text("name")).to_string() })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
