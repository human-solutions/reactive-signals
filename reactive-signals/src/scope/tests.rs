use std::rc::Rc;

use crate::{signal, tests::StringStore, Runtime};

#[test]
fn test_scopes_deep() {
    let rt = Runtime::new_client_side();
    let root = rt.new_root_scope();

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
    let rt = Runtime::new_client_side();
    let root = rt.new_root_scope();

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

// #[derive(Copy, Clone)]
// pub struct ScopeId<'rt> {
//     pub(crate) _sx: usize,
//     pub(crate) _rt: Runtime<'rt>,
// }

// impl<'rt> Scope<'rt> for ScopeId<'rt> {
//     fn get_id(&self) -> usize {
//         self._sx
//     }
// }
// pub trait Scope<'rt> {
//     fn get_id(&self) -> usize;
// }

// pub struct ScopeWrap1<I>
// where
//     I: for<'rt> Scope<'rt>,
// {
//     inner: I,
//     pos: usize,
// }

// impl<I> ScopeWrap1<I>
// where
//     I: for<'rt> Scope<'rt>,
// {
//     fn get_pos(&self) -> usize {
//         self.pos
//     }
// }

// impl<I: Scope<'rt>> Scope<'rt> for ScopeWrap1<'rt, I> {
//     fn get_id(&self) -> usize {
//         self.inner.get_id()
//     }
// }

// impl<'rt, I: Scope<'rt>> Deref for ScopeWrap1<'rt, I> {
//     type Target = I;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// pub struct ScopeWrap2<'rt, I>
// where
//     I: Scope<'rt>,
// {
//     inner: I,
//     some_data: usize,
// }

// impl<'rt, I: Scope<'rt>> Scope<'rt> for ScopeWrap2<'rt, I> {
//     fn get_id(&self) -> usize {
//         self.inner.get_id()
//     }
// }

// impl<'rt, I> ScopeWrap2<'rt, I>
// where
//     I: Scope<'rt>,
// {
//     #[allow(dead_code)]
//     fn get_some_data(&self) -> usize {
//         self.some_data
//     }
// }
// impl<'rt, I: Scope<'rt>> Deref for ScopeWrap2<'rt, I> {
//     type Target = I;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

// fn get_scope_id<'rt>() -> ScopeId<'rt> {
//     unimplemented!()
// }

// fn some_func<'rt, SC: Scope<'rt>>(_sc: SC) {}

// #[allow(dead_code)]
// fn chained_deref() {
//     let scope = get_scope_id();

//     let wrap1 = ScopeWrap1 {
//         inner: scope,
//         pos: 2,
//     };

//     wrap1.get_id();

//     let wrap2 = ScopeWrap2 {
//         inner: wrap1,
//         some_data: 4,
//     };

//     wrap2.get_id();
//     wrap2.get_pos();
//     wrap2.get_some_data();
//     some_func(wrap2);
// }
