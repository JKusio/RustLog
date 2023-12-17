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

    pub fn insert(&mut self, key: String, value: T) {
        let mut node: Box<Node<T>> = Box::new(Node::new(key, value));
        
        if self.root.is_none() {
            node.color = super::color::Color::Black;
            self.root = Some(node);
        } 
    }
}