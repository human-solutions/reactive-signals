#[cfg(test)]
mod tests;

#[allow(clippy::module_inception)]
mod scope;
mod scope_inner;

pub use scope::Scope;
pub(crate) use scope_inner::ScopeInner;
