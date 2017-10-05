use {Error, Result};
use std::str::FromStr;
use xmltree::Element;

pub trait Extension {
    fn xpath(&self, xpath: &str) -> Result<&Element>;
    fn as_str(&self) -> Result<&str>;
    fn convert<T>(&self) -> Result<T> where T: FromElement;
    fn xpath_then_parse<T>(&self, xpath: &str) -> Result<T>
        where T: FromStr,
              Error: From<<T as FromStr>::Err>
    {
        self.xpath(xpath).and_then(|e| e.as_str()).and_then(|s| s.parse().map_err(Error::from))
    }
}

pub trait FromElement {
    fn from_element(element: &Element) -> Result<Self> where Self: Sized;
}

impl Extension for Element {
    fn xpath(&self, xpath: &str) -> Result<&Element> {
        let mut element = self;
        for name in xpath.split('/') {
            if let Some(child) = element.get_child(name) {
                element = child;
            } else {
                return Err(Error::MissingChild(xpath.to_string()));
            }
        }
        Ok(element)
    }

    fn as_str(&self) -> Result<&str> {
        if let Some(s) = self.text.as_ref() {
            Ok(s)
        } else {
            Err(Error::NoText(self.clone()))
        }
    }

    fn convert<T>(&self) -> Result<T>
        where T: FromElement
    {
        T::from_element(self)
    }
}
