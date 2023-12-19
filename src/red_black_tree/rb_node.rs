use super::color::Color;

pub struct RBNode<'a, K, V> {
    pub key: K,
    pub value: V,
    pub color: Color,
    pub left: Option<Box<RBNode<'a, K, V>>>,
    pub right: Option<Box<RBNode<'a, K, V>>>,
    pub parent: Option<&'a RBNode<'a, K, V>>,
}
