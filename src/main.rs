use red_black_tree::rb_tree::RBTree;

mod red_black_tree;

fn main() {
    let tree: RBTree<i32, i32> = RBTree::new();

    if let None = tree.root {
        println!("Tree is empty");
    }
}
