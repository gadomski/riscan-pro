use nalgebra::Matrix4;

use {Error, Result};

pub fn matrix4_from_str(s: &str) -> Result<Matrix4<f64>> {
    let mut iter = s.split_whitespace();
    let mut iter_next = || {
        iter.next()
            .ok_or(Error::ParseMatrix(s.to_string()))
            .and_then(|s| s.parse().map_err(Error::from))
    };
    let m11 = try!(iter_next());
    let m21 = try!(iter_next());
    let m31 = try!(iter_next());
    let m41 = try!(iter_next());
    let m12 = try!(iter_next());
    let m22 = try!(iter_next());
    let m32 = try!(iter_next());
    let m42 = try!(iter_next());
    let m13 = try!(iter_next());
    let m23 = try!(iter_next());
    let m33 = try!(iter_next());
    let m43 = try!(iter_next());
    let m14 = try!(iter_next());
    let m24 = try!(iter_next());
    let m34 = try!(iter_next());
    let m44 = try!(iter_next());
    Ok(Matrix4::new(m11,
                    m21,
                    m31,
                    m41,
                    m12,
                    m22,
                    m32,
                    m42,
                    m13,
                    m23,
                    m33,
                    m43,
                    m14,
                    m24,
                    m34,
                    m44))
}

#[cfg(test)]
mod tests {
    use super::*;

    use nalgebra::{Eye, Matrix4};

    #[test]
    fn matrix4_from_str_ok() {
        let s = "1 2 3 4 0 1 0 0 0 0 1 0 0 0 0 1";
        let mut matrix = Matrix4::<f64>::new_identity(4);
        matrix[(0, 1)] = 2.;
        matrix[(0, 2)] = 3.;
        matrix[(0, 3)] = 4.;
        assert_eq!(matrix, matrix4_from_str(s).unwrap());
    }
}
