#![allow(dead_code)]

pub mod binary_tree {
    use std::convert::TryInto;
    pub struct BinaryTree {
        value: Option<i32>,
        left: Box<Option<BinaryTree>>,
        right: Box<Option<BinaryTree>>,
    }

    impl BinaryTree {
        pub fn new(value: Option<i32>, left: Option<BinaryTree>, right: Option<BinaryTree>) -> Self {
            BinaryTree { value: value, left: Box::new(left), right: Box::new(right) }
        }

        pub fn new_empty() -> Self {
            BinaryTree { value: None, left: Box::new(None), right: Box::new(None) }
        }
    }

    pub struct BinarySearchTree {
        value: Option<i32>,
        left: Box<Option<BinarySearchTree>>,
        right: Box<Option<BinarySearchTree>>,
    }

    impl BinarySearchTree {
        pub fn new(value: Option<i32>, left: Option<BinarySearchTree>, right: Option<BinarySearchTree>) -> Self {
            BinarySearchTree { value: value, left: Box::new(left), right: Box::new(right) }
        }

        pub fn new_empty() -> Self {
            BinarySearchTree { value: None, left: Box::new(None), right: Box::new(None) }
        }

        pub fn push(&mut self, value: i32) {
            match self.value {
                Some(x) => {
                    if value <= x {
                        match &mut*self.left {
                            Some(y) => y.push(value),
                            None => {
                                *self.left = Some(BinarySearchTree::new(Some(value), None, None));
                            },
                        }
                    } else {
                        match &mut*self.right {
                            Some(y) => y.push(value),
                            None => {
                                *self.right = Some(BinarySearchTree::new(Some(value), None, None));
                            },
                        }
                    }
                },
                None => self.value = Some(value),
            }
        }
    }

    pub fn inorder(tree: &BinaryTree) {
        match &*tree.left {
            Some(x) => inorder(x),
            None => (),
        }
        match tree.value {
            Some(x) => println!("{}", x),
            None => (),
        }
        match &*tree.right {
            Some(x) => inorder(x),
            None => (),
        }
    }

    pub fn preorder(tree: &BinaryTree) {
        match tree.value {
            Some(x) => println!("{}", x),
            None => (),
        }
        match &*tree.left {
            Some(x) => preorder(x),
            None => (),
        }
        match &*tree.right {
            Some(x) => preorder(x),
            None => (),
        }
    }

    pub fn postorder(tree: &BinaryTree) {
        match &*tree.left {
            Some(x) => postorder(x),
            None => (),
        }
        match &*tree.right {
            Some(x) => postorder(x),
            None => (),
        }
        match tree.value {
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    pub fn bst_inorder(tree: &BinarySearchTree) {
        match &*tree.left {
            Some(x) => bst_inorder(x),
            None => (),
        }
        match tree.value {
            Some(x) => println!("{}", x),
            None => (),
        }
        match &*tree.right {
            Some(x) => bst_inorder(x),
            None => (),
        }
    }
    pub fn bst_preorder(tree: &BinarySearchTree) {

    }
    pub fn bst_postorder(tree: &BinarySearchTree) {

    }

    pub fn tree_from_vec(values: &[i32]) -> BinaryTree {
        let len = values.len();
        if len == 1 {
            return BinaryTree::new(Some(values[0]), None, None);
        }
        if len == 2 {
            return BinaryTree::new(Some(values[len / 2]), 
                             Some(tree_from_vec(&values[..len/2])), 
                             None);
        }
        BinaryTree::new(Some(values[len / 2]), 
                  Some(tree_from_vec(&values[..len/2])), 
                  Some(tree_from_vec(&values[len/2+1..])))
    }
}

