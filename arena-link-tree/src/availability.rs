use std::ops::Range;

use crate::{Node, NodeId};

#[derive(Default)]
pub(crate) struct NodeSlotAvailability(pub(crate) Vec<u8>);

const FULL: u8 = u8::MAX;
const SLOT_SIZE: usize = 100;

impl NodeSlotAvailability {
    pub(crate) fn set_available(&mut self, id: NodeId) {
        set_available(SLOT_SIZE, &mut self.0, id.index());
    }

    pub(crate) fn get_available<T>(&mut self, vec: &[Node<T>]) -> Option<NodeId> {
        get_available(SLOT_SIZE, &mut self.0, |mut range| {
            range.end = usize::min(range.end, vec.len());
            for idx in range {
                if !vec[idx].is_used() && idx != 0 {
                    return Some(idx);
                }
            }
            None
        })
        .map(NodeId::from)
    }
}

#[inline]
fn set_available(slot_size: usize, slots: &mut Vec<u8>, idx: usize) {
    debug_assert!(slot_size < u8::MAX as usize);

    let slot_index = idx / slot_size;
    let sub_index = (idx % slot_size) as u8;

    while slots.len() <= slot_index {
        slots.push(FULL);
    }

    match slots[slot_index] {
        FULL => {
            slots[slot_index] = sub_index;
        }
        i if i > sub_index => {
            slots[slot_index] = sub_index;
        }
        _ => {}
    }
}

/// Find next available index and updates it to next one
#[inline]
fn get_available(
    slot_size: usize,
    slots: &mut Vec<u8>,
    next_available_index: impl FnOnce(Range<usize>) -> Option<usize>,
) -> Option<usize> {
    debug_assert!(slot_size < u8::MAX as usize);
    let slot = slots.iter().position(|&i| i != FULL)?;

    let sub_idx = slots[slot] as usize;

    let slot_start = slot * slot_size;

    let node = slot_start + sub_idx;

    if sub_idx + 1 >= slot_size {
        slots[slot] = FULL;
        return Some(node);
    }
    let range = (slot_start + sub_idx + 1)..(slot_start + slot_size);

    #[cfg(debug_assertions)]
    let range_dbg = range.clone();

    // update to next available slot
    if let Some(i) = next_available_index(range) {
        #[cfg(debug_assertions)]
        assert!(range_dbg.contains(&i));
        slots[slot] = (i % slot_size) as u8;
    } else {
        slots[slot] = FULL;
    }
    Some(node)
}

#[test]
fn test_available() {
    let mut slots = vec![FULL, FULL, FULL];
    const SLOT_SIZE: usize = 10;

    let found = get_available(SLOT_SIZE, &mut slots, |_| None);
    assert_eq!(found, None);

    set_available(10, &mut slots, 3);
    assert_eq!(format!("{slots:?}"), "[3, 255, 255]");

    let found = get_available(SLOT_SIZE, &mut slots, |_| Some(8));
    assert_eq!(format!("{found:?} {slots:?}"), "Some(3) [8, 255, 255]");

    let found = get_available(SLOT_SIZE, &mut slots, |_| Some(9));
    assert_eq!(format!("{found:?} {slots:?}"), "Some(8) [9, 255, 255]");

    let found = get_available(SLOT_SIZE, &mut slots, |r| {
        assert_eq!(r, 9..10);
        None
    });
    assert_eq!(format!("{found:?} {slots:?}"), "Some(9) [255, 255, 255]");

    set_available(SLOT_SIZE, &mut slots, 10);
    assert_eq!(format!("{slots:?}"), "[255, 0, 255]");

    let found = get_available(SLOT_SIZE, &mut slots, |_| Some(11));
    assert_eq!(format!("{found:?} {slots:?}"), "Some(10) [255, 1, 255]");

    let found = get_available(SLOT_SIZE, &mut slots, |r| Some(r.end - 1));
    assert_eq!(format!("{found:?} {slots:?}"), "Some(11) [255, 9, 255]");

    let found = get_available(SLOT_SIZE, &mut slots, |_| None);
    assert_eq!(format!("{found:?} {slots:?}"), "Some(19) [255, 255, 255]");

    set_available(SLOT_SIZE, &mut slots, 23);

    let found = get_available(SLOT_SIZE, &mut slots, |_| Some(27));
    assert_eq!(format!("{found:?} {slots:?}"), "Some(23) [255, 255, 7]");
}

#[test]
fn test_availability() {
    let mut availability = NodeSlotAvailability::default();
    let mut vec = Vec::new();

    for i in 0..1000 {
        let mut node = Node::new(i);
        node.parent = Some(NodeId::from(0));
        vec.push(node);
    }

    assert_eq!(availability.get_available(&vec), None);

    availability.set_available(NodeId::from(1));

    assert_eq!(availability.get_available(&vec), Some(NodeId::from(1)));
    assert_eq!(availability.get_available(&vec), None);
}
