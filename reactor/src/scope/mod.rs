#[cfg(test)]
mod tests;

mod scope;
mod scope_inner;

pub use scope::Scope;
pub(crate) use scope_inner::ScopeInner;
