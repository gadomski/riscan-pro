macro_rules! xpath {
    ($document:expr, $xpath:expr) => {{
        use sxd_xpath::{self, Value};   
        match sxd_xpath::evaluate_xpath($document, $xpath)? {
            Value::Nodeset(nodeset) => if nodeset.size() == 0 {
                return Err(Error::XpathNotFound($xpath.to_string()));
            } else {
                Value::Nodeset(nodeset)
            },
            value @ _ => value,
        }
    }}
}
