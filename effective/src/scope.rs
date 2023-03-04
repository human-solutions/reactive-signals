use arena_link_tree::NodeId;

use crate::{runtime_inner::RUNTIMES, Runtime};

#[cfg_attr(feature = "extra-traits", derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Scope {
    pub(crate) sx: NodeId,
    pub(crate) rt: Runtime,
}

impl Scope {
    pub fn create_root_scope() -> Self {
        let (rt, sx) = RUNTIMES.with(|rt| rt.borrow());
        Self { sx, rt }
    }
    pub fn new_child_scope(&self) -> Self {
        self.rt.with_mut(|rt| {
            let sx = rt.scopes.add_child(self.sx, Default::default());
            Self { sx, rt: rt.id }
        })
    }

    pub fn discard(self) {
        let (_discarded_scope_ids, is_root) = self.rt.with_mut(|rt| {
            (
                rt.scopes.reuse(self.sx, |s| s.reuse()),
                rt.scopes.root() == self.sx,
            )
        });
        if is_root {
            RUNTIMES.with(|rt| rt.return_to_pool(&self.rt));
        }
    }
}
