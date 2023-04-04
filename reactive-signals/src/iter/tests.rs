use insta::assert_snapshot;

use super::{IdVec, NodeResolver, VecTreeIter};

impl<'a> NodeResolver<'a> for Vec<Vec<usize>> {
    type Elem = &'a [usize];
    type Id = usize;

    fn node(&'a self, id: Self::Id) -> Self::Elem {
        &self[id]
    }
}

impl IdVec for &[usize] {
    type Output = usize;
    fn get(&self, idx: usize) -> Self::Output {
        self[idx]
    }
    fn len(&self) -> usize {
        (*self).len()
    }
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

#[test]
fn test_num_tree_iter() {
    let resolver: Vec<Vec<usize>> = vec![
        vec![1, 4], // 0
        vec![2, 3], // 1
        vec![],     // 2
        vec![],     // 3
        vec![5],    // 4
        vec![],     // 5
    ];

    let iter = VecTreeIter::new(&resolver, 0);
    let order = iter
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    assert_snapshot!(order, @"1, 2, 3, 4, 5");
}

#[test]
fn test_num_tree_iter_skip_children() {
    let resolver: Vec<Vec<usize>> = vec![
        vec![1, 4], // 0
        vec![2, 3], // 1
        vec![],     // 2
        vec![],     // 3
        vec![5],    // 4
        vec![],     // 5
    ];

    let mut iter = VecTreeIter::new(&resolver, 0);
    let mut nums = vec![];

    while let Some(next) = iter.next() {
        if next == 1 {
            iter.skip_children()
        }
        nums.push(next);
    }

    let order = nums
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    assert_snapshot!(order, @"1, 4, 5");
}
