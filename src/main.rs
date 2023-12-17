mod red_black_tree;
use red_black_tree::tree::RedBlackTree;

fn main() {
    let mut tree: RedBlackTree<String> = RedBlackTree::new();

    let key: String = String::from("key");
    let value: String = String::from("value");

    tree.insert(key, value);

    if let Some(node) = tree.get_root() {
        println!("root: {}", node.key);
    }
}
