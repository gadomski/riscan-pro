use nalgebra::Matrix4;
use xmltree::Element;

use Result;
use project::traits::GetDescendant;

#[derive(Clone, Debug, PartialEq)]
pub struct MountCalibration {
    matrix: Matrix4<f64>,
    name: String,
}

impl MountCalibration {
    pub fn from_element(element: &Element) -> Result<MountCalibration> {
        Ok(MountCalibration {
            matrix: try!(element.get_matrix4("matrix")),
            name: try!(element.get_text("name")).to_string(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn matrix(&self) -> Matrix4<f64> {
        self.matrix
    }
}
