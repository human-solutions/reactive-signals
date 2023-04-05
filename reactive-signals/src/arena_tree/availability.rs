use super::{flag_arr::FlagArr, Node, NodeId};

#[derive(Debug, Default)]
pub(crate) struct NodeSlotAvailability(pub(crate) FlagArr);

const SLOT_SIZE: usize = 16;

impl NodeSlotAvailability {
    #[inline]
    pub(crate) fn set_available(&mut self, id: NodeId) {
        set_available(&mut self.0, id.index());
    }

    #[inline]
    pub(crate) fn get_available<T>(&mut self, vec: &[Node<T>]) -> Option<NodeId> {
        get_available(&mut self.0, |i| !(i == 0 || vec[i].is_used())).map(NodeId::from)
    }

    #[inline]
    pub(crate) fn init(&mut self) -> NodeId {
        self.0.init();
        NodeId::root()
    }

    #[inline]
    pub(crate) fn discard(&mut self) {
        self.0.reset();
    }
}

#[inline]
fn set_available(flags: &mut FlagArr, idx: usize) {
    let slot_idx = idx / SLOT_SIZE;
    flags.set(slot_idx);
}

#[inline]
fn get_available(flags: &mut FlagArr, is_available: impl Fn(usize) -> bool) -> Option<usize> {
    let Some(slot_idx) = flags.take_last() else {
        return None;
    };
    let mut i = slot_idx * SLOT_SIZE;
    let slot_end = i + SLOT_SIZE;

    let mut found_idx = usize::MAX;

    while i < slot_end {
        if is_available(i) {
            found_idx = i;
            i += 1;
            break;
        }
        i += 1;
    }

    while i < slot_end {
        if is_available(i) {
            flags.set(slot_idx);
            return Some(found_idx);
        }
        i += 1;
    }
    (found_idx != usize::MAX).then_some(found_idx)
}

#[test]
fn test_set_available() {
    let mut flags = FlagArr::default();
    assert_eq!(format!("{flags:?}"), "[]");

    set_available(&mut flags, 0);
    assert_eq!(flags.arr[0], 0b1000_0000_0000_0000_0000_0000_0000_0000);

    flags.reset();
    set_available(&mut flags, 6);
    assert_eq!(flags.arr[0], 0b1000_0000_0000_0000_0000_0000_0000_0000);

    flags.reset();
    set_available(&mut flags, 15);
    assert_eq!(flags.arr[0], 0b1000_0000_0000_0000_0000_0000_0000_0000);

    flags.reset();
    set_available(&mut flags, 16);
    assert_eq!(flags.arr[0], 0b0100_0000_0000_0000_0000_0000_0000_0000);

    flags.reset();
    set_available(&mut flags, 31);
    assert_eq!(flags.arr[0], 0b0100_0000_0000_0000_0000_0000_0000_0000);

    flags.reset();
    let i = 32 * 16;
    set_available(&mut flags, i);
    assert_eq!(flags.arr[1], 0b1000_0000_0000_0000_0000_0000_0000_0000);
}

#[test]
fn test_get_available() {
    use std::cell::RefCell;

    let mut flags = FlagArr::default();
    let visited = RefCell::new(Vec::<String>::new());

    flags.set(0);

    let found = get_available(&mut flags, |i| {
        let mut v = visited.borrow_mut();
        v.push(format!("{i}"));
        debug_assert!(i < 100);
        2 == i || 4 == i
    });

    assert_eq!(visited.borrow().join(" "), "0 1 2 3 4");
    assert_eq!(found, Some(2));
    assert_eq!(flags.arr[0], 0b1000_0000_0000_0000_0000_0000_0000_0000);

    visited.borrow_mut().clear();

    let found = get_available(&mut flags, |i| {
        let mut v = visited.borrow_mut();
        v.push(format!("{i}"));
        0 == i
    });
    assert_eq!(
        visited.borrow().join(" "),
        "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15"
    );
    visited.borrow_mut().clear();

    assert_eq!(found, Some(0));
    assert_eq!(flags.arr[0], 0b0000_0000_0000_0000_0000_0000_0000_0000);

    flags.set(1);

    let found = get_available(&mut flags, |i| {
        let mut v = visited.borrow_mut();
        v.push(format!("{i}"));
        23 == i || 22 == i
    });

    assert_eq!(visited.borrow().join(" "), "16 17 18 19 20 21 22 23");
    visited.borrow_mut().clear();

    assert_eq!(found, Some(22));
    assert_eq!(flags.arr[0], 0b0100_0000_0000_0000_0000_0000_0000_0000);
}
