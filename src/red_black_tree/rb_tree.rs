use super::rb_node::RBNode;

pub struct RBTree<'a, K: Ord, V> {
    pub root: Option<Box<RBNode<'a, K, V>>>,
}

impl<'a, K: Ord, V> RBTree<'a, K, V> {
    pub fn new() -> RBTree<'a, K, V> {
        RBTree { root: None }
    }
}
