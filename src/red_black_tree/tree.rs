use super::node::Node;

pub struct RedBlackTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> RedBlackTree<T> {
    pub fn new() -> RedBlackTree<T> {
        RedBlackTree { root: None }
    }   

    pub fn get_root(&self) -> Option<&Box<Node<T>>> {
        self.root.as_ref()
    }
}