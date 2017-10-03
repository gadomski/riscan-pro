use Result;
use nalgebra::Matrix4;

pub fn matrix_from_str(s: &str) -> Result<Matrix4<f64>> {
    use Error;
    s.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()
        .map(|v| Matrix4::from_iterator(v).transpose())
}
