#![allow(dead_code)]
#[derive(Debug)]
pub(crate) enum ArrVec<const N: usize, T: Ord + Eq + Copy> {
    Arr([Option<T>; N]),
    Vec(Vec<T>),
}

impl<const N: usize, T: Ord + Eq + Copy> Default for ArrVec<N, T> {
    fn default() -> Self {
        Self::Arr([None; N])
    }
}

const DEBUG: bool = false;

#[inline]
fn to_vec<const N: usize, T: Ord + Eq + Copy>(arr: &[Option<T>; N], elem: T) -> ArrVec<N, T> {
    let mut vec: Vec<T> = arr.iter().filter_map(|val| val.map(|t| t)).collect();
    vec.push(elem);
    ArrVec::Vec(vec)
}

impl<const N: usize, T: Ord + Eq + Copy> ArrVec<N, T> {
    #[inline]
    pub(crate) fn insert(&mut self, insrt: T) {
        if let Some(vec) = self.insert_or_upgrade(insrt) {
            *self = vec;
        }
    }

    #[inline]
    fn insert_or_upgrade(&mut self, insrt: T) -> Option<Self> {
        match self {
            Self::Arr(ref mut arr) => {
                let mut carry_forwards: Option<T> = None;
                for i in 0..N {
                    let last = i == N - 1;
                    if last {
                        // we are at the last index
                        match (arr[i], carry_forwards) {
                            (Some(entry), Some(fwrd)) => {
                                DEBUG.then(|| println!("[{i} ] to_vec fwrd"));
                                // elem already inserted and we carry forwards but there is an overflow
                                arr[i] = Some(fwrd);
                                return Some(to_vec(arr, entry));
                            }
                            (Some(entry), None) if entry == insrt => {
                                DEBUG.then(|| println!("[{i} ] == insrt"));
                                return None;
                            }
                            (Some(entry), None) if entry > insrt => {
                                DEBUG.then(|| println!("[{i} ] to_vec entry"));
                                // last position is where the elem should be inserted but it's occupied.
                                arr[i] = Some(insrt);
                                return Some(to_vec(arr, entry));
                            }
                            (Some(_), None) => {
                                DEBUG.then(|| println!("[{i} ] to_vec insrt"));
                                return Some(to_vec(arr, insrt)); // the elem should be added last
                            }
                            (None, None) => {
                                DEBUG.then(|| println!("[{i} ] = insrt"));
                                arr[i] = Some(insrt)
                            }
                            (None, Some(fwrd)) => {
                                DEBUG.then(|| println!("[{i} ] = fwrd"));
                                arr[i] = Some(fwrd)
                            }
                        }
                    } else {
                        // there are more indexes remaining
                        match (arr[i], carry_forwards) {
                            (Some(entry), Some(frwd)) => {
                                DEBUG.then(|| println!("[{i}>] fwrd"));
                                carry_forwards = Some(entry);
                                arr[i] = Some(frwd);
                            }
                            (Some(entry), None) if entry == insrt => {
                                DEBUG.then(|| println!("[{i}>] == insrt"));
                                return None;
                            }
                            (Some(entry), None) if entry > insrt => {
                                // insertion point found, and already occupied
                                DEBUG.then(|| println!("[{i}>] insrt + fwrd"));
                                carry_forwards = Some(entry);
                                arr[i] = Some(insrt);
                            }
                            (Some(_), None) => {
                                DEBUG.then(|| println!("[{i}>]"));
                            } // not insertion point and now forwarding, keep going
                            (None, Some(frwd)) => {
                                DEBUG.then(|| println!("[{i}>] = fwrd"));

                                arr[i] = Some(frwd);
                                return None;
                            }
                            (None, None) => {
                                DEBUG.then(|| println!("[{i}>] = insrt"));
                                arr[i] = Some(insrt);
                                return None;
                            }
                        }
                    }
                }
            }
            Self::Vec(vec) => {
                match vec.binary_search(&insrt) {
                    Ok(_) => {} // already present
                    Err(index) => vec.insert(index, insrt),
                }
            }
        }
        None
    }

    #[inline]
    pub(crate) fn clear(&mut self) {
        match self {
            Self::Arr(a) => a.iter_mut().for_each(|entry| *entry = None),
            Self::Vec(v) => v.clear(),
        }
    }

    #[inline]
    pub(crate) fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        match self {
            Self::Arr(ref mut arr) => {
                let mut next: usize = 0;
                for i in 0..N {
                    if let Some(elem) = arr[i] {
                        if f(&elem) {
                            arr[next] = Some(elem);
                            next += 1;
                        }
                        if i + 1 != next {
                            arr[i] = None;
                        }
                    } else {
                        return;
                    }
                }
            }
            Self::Vec(v) => v.retain(f),
        }
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Arr(a) => a.iter().filter(|e| e.is_some()).count(),
            Self::Vec(v) => v.len(),
        }
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Arr(a) => !a.iter().any(|elem| elem.is_some()),
            Self::Vec(v) => v.is_empty(),
        }
    }

    #[inline]
    pub(crate) fn get(&self, index: usize) -> T {
        match self {
            Self::Arr(a) => a[index].unwrap(),
            Self::Vec(v) => v[index],
        }
    }
}

#[test]
fn test_insert_incr() {
    let mut av = ArrVec::<3, usize>::default();

    av.insert(1);
    assert_eq!(format!("{av:?}"), "Arr([Some(1), None, None])");

    av.insert(2);
    assert_eq!(format!("{av:?}"), "Arr([Some(1), Some(2), None])");

    av.insert(3);
    assert_eq!(format!("{av:?}"), "Arr([Some(1), Some(2), Some(3)])");

    av.insert(4);
    assert_eq!(format!("{av:?}"), "Vec([1, 2, 3, 4])");
}

#[test]
fn test_insert_decr() {
    let mut av = ArrVec::<3, usize>::default();

    av.insert(4);
    assert_eq!(format!("{av:?}"), "Arr([Some(4), None, None])");

    av.insert(3);
    assert_eq!(format!("{av:?}"), "Arr([Some(3), Some(4), None])");

    av.insert(2);
    assert_eq!(format!("{av:?}"), "Arr([Some(2), Some(3), Some(4)])");

    av.insert(1);
    assert_eq!(format!("{av:?}"), "Vec([1, 2, 3, 4])");
}

#[test]
fn test_retain_even() {
    let mut av = ArrVec::<4, usize>::default();

    av.insert(1);
    av.insert(2);
    av.insert(3);
    av.insert(4);

    assert_eq!(
        format!("{av:?}"),
        "Arr([Some(1), Some(2), Some(3), Some(4)])"
    );

    av.retain(|entry| entry % 2 == 0);

    assert_eq!(format!("{av:?}"), "Arr([Some(2), Some(4), None, None])");
}

#[test]
fn test_retain_odd() {
    let mut av = ArrVec::<4, usize>::default();

    av.insert(1);
    av.insert(2);
    av.insert(3);
    av.insert(4);

    assert_eq!(
        format!("{av:?}"),
        "Arr([Some(1), Some(2), Some(3), Some(4)])"
    );

    av.retain(|entry| entry % 2 != 0);

    assert_eq!(format!("{av:?}"), "Arr([Some(1), Some(3), None, None])");
}
