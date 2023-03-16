use arena_link_tree::NodeId;

use crate::{runtime_inner::RUNTIMES, Runtime};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Scope {
    pub(crate) sx: NodeId,
    pub(crate) rt: Runtime,
}

impl Scope {
    #[allow(unused)]
    #[cfg(feature = "profile")]
    pub fn bench_root() -> Self {
        RUNTIMES.with(|pool| pool.bench_clean_all());
        Self::new_root()
    }

    pub fn new_root() -> Self {
        let (rt, sx) = RUNTIMES.with(|rt| rt.borrow_rt());
        Self { sx, rt }
    }
    pub fn new_child(&self) -> Self {
        self.rt.with_mut(|rt| {
            let sx = rt.scope_tree.add_child(self.sx, Default::default());
            Self { sx, rt: rt.id }
        })
    }

    pub fn discard(self) {
        let is_root = self.rt.with_mut(|rt| {
            let discarded = rt.scope_tree.discard(self.sx, |s| s.reuse());
            rt.scope_tree
                .iter_mut_from(rt.scope_tree.root())
                .for_each(|tree, node| tree[node].remove_scopes(&discarded));
            rt.scope_tree.root() == self.sx
        });
        if is_root {
            RUNTIMES.with(|rt| rt.return_rt(&self.rt));
        }
    }
}
