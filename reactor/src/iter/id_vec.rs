pub(crate) trait IdVec {
    type Output;
    fn get(&self, idx: usize) -> Self::Output;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub(crate) struct IdVecIter<V: IdVec> {
    vec: V,
    pos: usize,
}

impl<V: IdVec> IdVecIter<V> {
    pub(crate) fn new(vec: V) -> Self {
        Self {
            vec,
            pos: usize::MAX,
        }
    }

    pub(crate) fn remaining(&self) -> usize {
        if self.pos == usize::MAX {
            self.vec.len()
        } else {
            self.vec.len() - self.pos - 1
        }
    }

    pub(crate) fn has_more(&self) -> bool {
        if self.pos == usize::MAX {
            !self.vec.is_empty()
        } else {
            self.pos < (self.vec.len() - 1)
        }
    }
}

impl<V: IdVec> Iterator for IdVecIter<V> {
    type Item = V::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == usize::MAX {
            if self.vec.is_empty() {
                None
            } else {
                self.pos = 0;
                Some(self.vec.get(0))
            }
        } else if self.pos + 1 >= self.vec.len() {
            None
        } else {
            self.pos += 1;
            let id = self.vec.get(self.pos);
            Some(id)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }
}

impl<V: IdVec> ExactSizeIterator for IdVecIter<V> {
    fn len(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]
impl IdVec for Vec<usize> {
    type Output = usize;
    fn get(&self, idx: usize) -> Self::Output {
        self[idx]
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[test]
fn test_id_vec() {
    assert_eq!(IdVecIter::new(vec![]).next(), None);

    let mut iter = IdVecIter::new(vec![0, 1, 2]);
    assert_eq!(iter.remaining(), 3);

    let val = iter.next();
    assert_eq!(val, Some(0));
    assert_eq!(iter.remaining(), 2);
    assert_eq!(iter.has_more(), true);

    let val = iter.next();
    assert_eq!(val, Some(1));
    assert_eq!(iter.has_more(), true);
    assert_eq!(iter.remaining(), 1);

    let val = iter.next();
    assert_eq!(val, Some(2));
    assert_eq!(iter.has_more(), false);
    assert_eq!(iter.remaining(), 0);

    let val = iter.next();
    assert_eq!(val, None);
    assert_eq!(iter.has_more(), false);
    assert_eq!(iter.remaining(), 0);
}
