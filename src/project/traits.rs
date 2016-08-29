use std::iter::FromIterator;
use std::str::FromStr;

use nalgebra::Matrix4;
use xmltree::Element;

use {Error, Result};
use utils;

pub trait GetDescendant {
    fn get_descendant(&self, name: &str) -> Result<&Element>;
    fn get_children(&self, name: &str) -> Result<&Vec<Element>> {
        self.get_descendant(name).map(|e| &e.children)
    }
    fn get_text(&self, name: &str) -> Result<&str> {
        self.get_descendant(name)
            .and_then(|e| {
                e.text.as_ref().map(|s| s.as_str()).ok_or(Error::MissingElement(name.to_string()))
            })
    }
    fn get_noderef(&self, name: &str) -> Result<&str> {
        self.get_descendant(name).and_then(|e| {
            e.attributes
                .get("noderef")
                .and_then(|s| s.split('/').last())
                .ok_or(Error::MissingElement(format!("{}.noderef", name)))
        })
    }
    fn get_matrix4(&self, name: &str) -> Result<Matrix4<f64>> {
        self.get_text(name).and_then(|s| utils::matrix4_from_str(s))
    }
    fn parse<T>(&self, name: &str) -> Result<T>
        where T: FromStr,
              Error: From<<T as FromStr>::Err>
    {
        self.get_text(name).and_then(|s| s.parse().map_err(Error::from))
    }
    fn map_children<F, A, B>(&self, name: &str, function: F) -> Result<A>
        where F: Fn(&Element) -> Result<B>,
              A: FromIterator<B>
    {
        self.get_children(name).and_then(|children| {
            children.iter()
                .map(function)
                .collect()
        })
    }
}

impl GetDescendant for Element {
    fn get_descendant(&self, name: &str) -> Result<&Element> {
        let mut element = self;
        for name in name.split('/') {
            match element.get_child(name) {
                Some(child) => element = child,
                None => return Err(Error::MissingElement(name.to_string())),
            }
        }
        Ok(element)
    }
}
