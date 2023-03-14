#[cfg(test)]
mod tests;

mod id_vec;
mod ref_vec_elem;
mod signal_iter;
mod vec_tree_iter;

pub(crate) use id_vec::{IdVec, IdVecIter};
pub(crate) use ref_vec_elem::RefVecElem;
pub(crate) use vec_tree_iter::{ChildVecResolver, VecTreeIter};

const DEBUG: bool = false;
