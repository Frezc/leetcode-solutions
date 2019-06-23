#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

/** copy
impl ListNode {
    pub fn to_vec(&self) -> Vec<i32> {
        let mut node = Some(self);
        let mut result = vec![];
        while let Some(n) = node {
            result.push(n.val);
            node = n.next.as_ref().map(|b| b.as_ref());
        }
        result
    }
}
*/

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}

pub fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    to_list(Vec::from(arr))
}


#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}