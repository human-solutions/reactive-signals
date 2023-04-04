use std::{marker::PhantomData, ops::Deref, rc::Rc};

use crate::{
    runtimes::{Runtime, ServerRuntime},
    signal,
    tests::StringStore,
};

#[test]
fn test_scopes_deep() {
    let root = ServerRuntime::new_root_scope();

    let mut sc = root.clone();
    let num_sig = signal!(sc, 5);

    (0..3).for_each(|_| sc = sc.new_child());

    let output = Rc::new(StringStore::new());
    let _str_sig = signal!(sc, clone: output, move || output
        .push(format!("val: {}", num_sig.get())));

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[test]
fn test_scopes_discard() {
    let root = ServerRuntime::new_root_scope();

    let sc0 = root.clone();
    let num_sig = signal!(sc0, 5);

    let sc1 = sc0.new_child();
    let sc2 = sc1.new_child();
    let sc3 = sc2.new_child();

    let output = Rc::new(StringStore::new());
    let _str_sig = signal!(sc3, clone: output, move || output
        .push(format!("val: {}", num_sig.get())));

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");

    sc2.discard();

    num_sig.set(4);

    assert_eq!(output.values(), "val: 5, val: 4");
}

#[derive(Copy, Clone)]
pub struct ScopeId<RT: Runtime> {
    pub(crate) _sx: usize,
    pub(crate) _rt: RT,
}

impl<RT: Runtime> Scope<RT> for ScopeId<RT> {
    fn get_id(&self) -> usize {
        self._sx
    }
}
pub trait Scope<RT: Runtime> {
    fn get_id(&self) -> usize;
}

pub struct ScopeWrap1<I, RT>
where
    I: Scope<RT>,
    RT: Runtime,
{
    inner: I,
    pos: usize,
    rt: PhantomData<RT>,
}

impl<I, RT> ScopeWrap1<I, RT>
where
    I: Scope<RT>,
    RT: Runtime,
{
    fn get_pos(&self) -> usize {
        self.pos
    }
}

impl<I: Scope<RT>, RT: Runtime> Scope<RT> for ScopeWrap1<I, RT> {
    fn get_id(&self) -> usize {
        self.inner.get_id()
    }
}

impl<I: Scope<RT>, RT: Runtime> Deref for ScopeWrap1<I, RT> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct ScopeWrap2<I, RT>
where
    I: Scope<RT>,
    RT: Runtime,
{
    inner: I,
    some_data: usize,
    rt: PhantomData<RT>,
}

impl<I: Scope<RT>, RT: Runtime> Scope<RT> for ScopeWrap2<I, RT> {
    fn get_id(&self) -> usize {
        self.inner.get_id()
    }
}

impl<I, RT> ScopeWrap2<I, RT>
where
    I: Scope<RT>,
    RT: Runtime,
{
    #[allow(dead_code)]
    fn get_some_data(&self) -> usize {
        self.some_data
    }
}
impl<I: Scope<RT>, RT: Runtime> Deref for ScopeWrap2<I, RT> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn get_scope_id<RT: Runtime>() -> ScopeId<RT> {
    unimplemented!()
}

fn some_func<RT: Runtime, SC: Scope<RT>>(_sc: SC) {}

#[allow(dead_code)]
fn chained_deref() {
    let scope = get_scope_id::<ServerRuntime>();

    let wrap1 = ScopeWrap1 {
        inner: scope,
        pos: 2,
        rt: PhantomData,
    };

    wrap1.get_id();

    let wrap2 = ScopeWrap2 {
        inner: wrap1,
        some_data: 4,
        rt: PhantomData,
    };

    wrap2.get_id();
    wrap2.get_pos();
    wrap2.get_some_data();
    some_func(wrap2);
}
