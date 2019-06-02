use std::rc::Rc;
use std::cell::{RefCell, Ref};
use std::collections::vec_deque::VecDeque;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn to_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if v.len() == 0 {
        return None;
    }
    let head = Some(Rc::new(RefCell::new(TreeNode::new(v[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().map(|n|n.clone()));
    for chunk in v[1..].chunks(2) {
        let parent = queue.pop_front();
        if let Some(parent) = parent.unwrap() {
            if let Some(left) = chunk[0] {
                let left = Rc::new(RefCell::new(TreeNode::new(left)));
                parent.borrow_mut().left = Some(left.clone());
                queue.push_back(Some(left));
            }
            if let Some(Some(right)) = chunk.get(1) {
                let right = Rc::new(RefCell::new(TreeNode::new(*right)));
                parent.borrow_mut().right = Some(right.clone());
                queue.push_back(Some(right));
            }
        }
    }
    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}