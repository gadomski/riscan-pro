use {Result, Transform3};

pub fn transform_from_str(s: &str) -> Result<Transform3> {
    use Error;
    use nalgebra::Matrix4;
    s.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()
        .map(|v| Transform3::from_matrix_unchecked(Matrix4::from_iterator(v).transpose()))
}
