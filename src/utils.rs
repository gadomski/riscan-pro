use {Projective3, Result};

pub fn projective_from_str(s: &str) -> Result<Projective3> {
    use Error;
    use nalgebra::{self, Matrix4};
    s.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()
        .and_then(|v| {
            if v.len() != 16 {
                return Err(Error::ParseMatrix4(s.to_string()));
            }
            let matrix = Matrix4::from_iterator(v).transpose();
            nalgebra::try_convert(matrix).ok_or(Error::Inverse(matrix))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error() {
        assert!(projective_from_str("").is_err());
    }
}
