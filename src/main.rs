#![allow(dead_code)]

struct Tree {
    value: i32,
    children: Vec<Box<Option<Tree>>>,
}

impl Tree {
    fn new(value: i32, children: Vec<Box<Option<Tree>>>) -> Tree {
        Tree {value: value, children: children}
    }
}

fn construct_tree() -> Tree {
    let child1 = Box::new(Some(Tree::new(1, Vec::new())));
    let child2 = Box::new(Some(Tree::new(2, Vec::new())));
    Tree::new(3, vec![child1, child2])
}

mod tree;

pub use crate::tree::binary_tree;

fn main() {
    let vec = vec![1, 2, 3, 4, 5, 6];
    let tree = binary_tree::tree_from_vec(&vec);
    /*
    binary_tree::inorder(&tree);
    println!("------");
    binary_tree::preorder(&tree);
    println!("------");
    binary_tree::postorder(&tree);
    */
    let mut bst = binary_tree::BinarySearchTree::new_empty();
    bst.push(5);
    bst.push(3);
    bst.push(1);
    bst.push(8);
    bst.push(7);
    binary_tree::bst_inorder(&bst);
    println!("{}", bst.size());
}
