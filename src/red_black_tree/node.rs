use super::color::Color;

pub struct Node<T> {
    key: String,
    value: T,
    color: Color,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    parent: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(key: String, value: T, color: Color) -> Node<T> {
        Node {
            key,
            value,
            color,
            left: None,
            right: None,
            parent: None,
        }
    }
}