mod red_black_tree;
use red_black_tree::{tree::RedBlackTree, node::Node};

fn main() {
    let tree: RedBlackTree<String> = RedBlackTree::new();
    let root: Option<&Box<Node<String>>> = tree.get_root();

    if root.is_none() {
        println!("Root is None");
    } else {
        println!("Root is not None");
    }
}
