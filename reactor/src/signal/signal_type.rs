pub trait SignalType {
    type Inner;
    fn is_eq(&self, _other: &Self::Inner) -> bool {
        false
    }
    fn opt_hash(&self) -> Option<u64> {
        None
    }

    fn inner(&self) -> &Self::Inner;
    fn inner_mut(&mut self) -> &mut Self::Inner;
}

#[cfg(test)]
fn set<T: 'static + SignalType>(val1: &T, val2: &T::Inner) -> bool {
    val1.is_eq(&val2)
}

#[test]
fn cmp_test() {
    use crate::signal::{Data, EqData};

    let d1 = Data(3);
    let d2 = Data(2);

    assert_eq!(set(&d1, &d2.inner()), false);
    assert_eq!(set(&d1, &d1.inner()), false);

    let d1 = EqData(3);
    let d2 = EqData(2);

    assert_eq!(set(&d1, &d2.inner()), false);
    assert_eq!(set(&d1, &d1.inner()), true);
}
