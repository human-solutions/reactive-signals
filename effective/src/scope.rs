use slotmap::new_key_type;

use crate::{runtime::Runtime, signal::SignalId};

new_key_type! { pub struct ScopeId; }

#[cfg_attr(feature = "extra-traits", derive(Debug))]
pub struct Scope {
    parent: Option<ScopeId>,
    children: Vec<ScopeId>,

    /// The signals that are defined in this scope.
    signals: Vec<SignalId>,

    /// Keep a list of used signals, so that we can unsubscribe
    /// when the scope is dropped.
    dependencies: Vec<SignalId>,
}

impl Scope {
    pub(crate) fn root() -> Self {
        Self {
            parent: None,
            children: vec![],
            signals: vec![],
            dependencies: vec![],
        }
    }
}
