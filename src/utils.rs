//! Utility functions.

use Result;
use nalgebra::Projective3;
use std;
use std::io::Write;

/// Parse a projective3 matrix from whitespace-delimited text.
pub fn parse_projective3(s: &str) -> Result<Projective3<f64>> {
    use nalgebra::{self, Matrix4};
    use Error;

    let numbers = s.split_whitespace()
        .map(|s| s.parse::<f64>().map_err(Error::from))
        .collect::<Result<Vec<_>>>()?;
    if numbers.len() != 16 {
        Err(Error::ParseProjective3(s.to_string()))
    } else {
        nalgebra::try_convert(Matrix4::from_iterator(numbers).transpose())
            .ok_or_else(|| Error::ParseProjective3(s.to_string()))
    }
}

/// Writes a projective3 to a `Write`.
pub fn write_projective3<W: Write>(mut write: W, matrix: &Projective3<f64>) -> std::io::Result<()> {
    for row in 0..4 {
        for col in 0..4 {
            write!(write, "{}", matrix[(row, col)])?;
            if col < 3 {
                write!(write, " ")?;
            }
        }
        writeln!(write, "")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projective3() {
        let matrix = parse_projective3("-0.010877741999999997 -0.003724941 -0.999933898 0.18508641   0.019274697 0.999806486 -0.0039341460000000013 0.000460517   0.99975505 -0.019316217 -0.01080384 -0.092802787   0 0 0 1").unwrap();
        assert_relative_eq!(-0.003724941, matrix[(0, 1)], epsilon = 1e-9);
        assert_relative_eq!(0.18508641, matrix[(0, 3)], epsilon = 1e-8);
        assert_eq!(
            Projective3::identity(),
            parse_projective3("1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1").unwrap()
        );
        assert!(parse_projective3("1 0 0 0 0 1 0 0 0 0 1 0 0 0 1").is_err());
        assert!(parse_projective3("1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1 0").is_err());
    }
}
