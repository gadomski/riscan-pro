//! Improvements to `xmltree::Element`.

use {Error, Result};
use std::str::FromStr;
use xmltree::Element;

/// An extension to the xmltree element to make it more ergonomic.
pub trait Extension {
    /// Returns a child element by slash-seperated names, or an error if the path does not exist.
    ///
    /// # Examples
    ///
    /// `Extension` is implemented for `xmltree::Element`:
    ///
    /// ```
    /// extern crate xmltree;
    /// # extern crate riscan_pro;
    /// # fn main () {
    /// use xmltree::Element;
    /// use riscan_pro::element::Extension;
    /// use std::fs::File;
    ///
    /// let file = File::open("data/project.RiSCAN/project.rsp").unwrap();
    /// let element = Element::parse(file).unwrap();
    /// let pop_matrix = element.child("pop/matrix").unwrap();
    /// # }
    /// ```
    fn child(&self, path: &str) -> Result<&Element>;

    /// Returns a vector of children, as selected by name.
    ///
    /// # Examples
    ///
    /// `Extension` is implemented for `xmltree::Element`:
    ///
    /// ```
    /// extern crate xmltree;
    /// # extern crate riscan_pro;
    /// # fn main () {
    /// use xmltree::Element;
    /// use riscan_pro::element::Extension;
    /// use std::fs::File;
    ///
    /// let file = File::open("data/project.RiSCAN/project.rsp").unwrap();
    /// let element = Element::parse(file).unwrap();
    /// let relector_clibrations = element.children("calibrations/reflcalibs/reflcalib").unwrap();
    /// # }
    /// ```
    fn children(&self, path: &str) -> Result<&Vec<Element>>;

    /// Returns this element's inner text as a string, or returns an error if there is no text.
    ///
    /// # Examples
    ///
    /// `Extension` is implemented for `xmltree::Element`:
    ///
    /// ```
    /// extern crate xmltree;
    /// # extern crate riscan_pro;
    /// # fn main () {
    /// use xmltree::Element;
    /// use riscan_pro::element::Extension;
    /// use std::fs::File;
    ///
    /// let file = File::open("data/project.RiSCAN/project.rsp").unwrap();
    /// let element = Element::parse(file).unwrap();
    /// let pop_matrix_str = element.child("pop/matrix").unwrap().as_str().unwrap();
    /// # }
    /// ```
    fn as_str(&self) -> Result<&str>;

    /// Returns this element's nodref attribute as a string.
    ///
    /// The noderef is trimmed to only use the last element.
    ///
    /// # Examples
    ///
    /// `Extension` is implemented for `xmltree::Element`:
    ///
    /// ```
    /// extern crate xmltree;
    /// # extern crate riscan_pro;
    /// # fn main () {
    /// use xmltree::Element;
    /// use riscan_pro::element::Extension;
    /// use std::fs::File;
    ///
    /// let file = File::open("data/project.RiSCAN/project.rsp").unwrap();
    /// let element = Element::parse(file).unwrap();
    /// let logo = element
    ///     .child("collections/overlays/overlay/overlayitem/source_ref")
    ///     .unwrap()
    ///     .noderef()
    ///     .unwrap();
    /// assert_eq!("Logo", logo);
    /// # }
    /// ```
    fn noderef(&self) -> Result<&str>;

    /// Parses this element's inner text, or returns an error if there is no text or if the parse
    /// fails.
    ///
    /// # Examples
    ///
    /// `Extension` is implemented for `xmltree::Element`:
    ///
    /// ```
    /// extern crate xmltree;
    /// # extern crate riscan_pro;
    /// # fn main () {
    /// use xmltree::Element;
    /// use riscan_pro::element::Extension;
    /// use std::fs::File;
    ///
    /// let file = File::open("data/project.RiSCAN/project.rsp").unwrap();
    /// let element = Element::parse(file).unwrap();
    /// let version: f64 = element.child("app_version").unwrap().parse_text().unwrap();
    /// # }
    /// ```
    fn parse_text<T>(&self) -> Result<T>
    where
        T: FromStr,
        Error: From<<T as FromStr>::Err>,
    {
        self.as_str().and_then(|s| s.parse().map_err(Error::from))
    }
}

impl Extension for Element {
    fn child(&self, path: &str) -> Result<&Element> {
        burrow(self, path.split('/'))
    }

    fn children(&self, path: &str) -> Result<&Vec<Element>> {
        let mut iter = path.split('/').rev();
        let last = iter.next().ok_or_else(|| {
            Error::MissingChild(self.name.clone(), String::new())
        })?;
        let element = burrow(self, iter.rev())?;
        if element.children.iter().all(|child| child.name == last) {
            Ok(&element.children)
        } else {
            Err(Error::MissingChild(element.name.clone(), last.to_string()))
        }
    }

    fn as_str(&self) -> Result<&str> {
        self.text.as_ref().map(|s| s.as_str()).ok_or_else(|| {
            Error::NoElementText(self.clone())
        })
    }

    fn noderef(&self) -> Result<&str> {
        if let Some(noderef) = self.attributes.get("noderef").and_then(
            |s| s.split('/').last(),
        )
        {
            Ok(noderef)
        } else {
            Err(Error::MissingNoderef(self.clone()))
        }
    }
}

fn burrow<'a, I: Iterator<Item = &'a str>>(mut element: &Element, iter: I) -> Result<&Element> {
    for name in iter {
        if let Some(child) = element.get_child(name) {
            element = child;
        } else {
            return Err(Error::MissingChild(element.name.clone(), name.to_string()));
        }
    }
    Ok(element)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project() -> Element {
        use std::fs::File;
        Element::parse(File::open("data/project.RiSCAN/project.rsp").unwrap()).unwrap()
    }

    #[test]
    fn child() {
        let project = project();
        assert_eq!("pop", project.child("pop").unwrap().name);
        assert_eq!("matrix", project.child("pop/matrix").unwrap().name);
        assert_eq!(
            "reflcalib",
            project
                .child("calibrations/reflcalibs/reflcalib")
                .unwrap()
                .name
        );
        assert!(project.child("").is_err());
        assert!(project.child("not-an-element").is_err());
    }

    #[test]
    fn children() {
        let project = project();
        let reflectors = project
            .children("calibrations/reflcalibs/reflcalib")
            .unwrap();
        assert_eq!(4, reflectors.len());
        assert!(project.children("pop").is_err());
        assert!(project.children("").is_err());
        assert!(project.children("not-an-element").is_err());
    }

    #[test]
    fn as_str() {
        let project = project();
        assert!(project.as_str().is_err());
        assert_eq!(
            "RiSCAN PRO",
            project.child("app_caption").unwrap().as_str().unwrap()
        );
    }

    #[test]
    fn noderef() {
        let project = project();
        assert_eq!(
            "Logo",
            project
                .child("collections/overlays/overlay/overlayitem/source_ref")
                .unwrap()
                .noderef()
                .unwrap()
        );
        assert_eq!(
            "Infratec_VarioCAM_HD_15mm_11-16-2015_Preston",
            project
                .child("geometry_objects/images/image/camcalib_ref")
                .unwrap()
                .noderef()
                .unwrap()
        );
    }
}
