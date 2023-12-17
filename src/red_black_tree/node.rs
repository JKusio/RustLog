use super::color::Color;

pub struct Node<T> {
    pub key: String,
    pub value: T,
    pub color: Color,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
    pub parent: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(key: String, value: T) -> Node<T> {
        Node {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
        }
    }
} 