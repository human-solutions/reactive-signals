use leptos::{
    create_isomorphic_effect, create_memo, create_scope, create_signal, RuntimeId, SignalGet,
    SignalUpdate,
};
use std::{cell::Cell, rc::Rc};

pub fn leptos_create_1000_signals(runtime: RuntimeId) {
    create_scope(runtime, |sc| {
        let sigs = (0..1000)
            .map(|n| create_signal(sc, n).0)
            .collect::<Vec<_>>();
        let memo = create_memo(sc, move |_| sigs.iter().map(|r| r.get()).sum::<i32>());
        assert_eq!(memo.get(), 499500);
    })
    .dispose()
}

pub fn leptos_create_and_update_1000_signals(runtime: RuntimeId) {
    create_scope(runtime, |sc| {
        let acc = Rc::new(Cell::new(0));
        let sigs = (0..1000).map(|n| create_signal(sc, n)).collect::<Vec<_>>();
        let reads = sigs.iter().map(|(r, _)| *r).collect::<Vec<_>>();
        let writes = sigs.iter().map(|(_, w)| *w).collect::<Vec<_>>();
        let memo = create_memo(sc, move |_| reads.iter().map(|r| r.get()).sum::<i32>());
        assert_eq!(memo.get(), 499500);
        create_isomorphic_effect(sc, {
            let acc = Rc::clone(&acc);
            move |_| {
                acc.set(memo.get());
            }
        });
        assert_eq!(acc.get(), 499500);

        writes[1].update(|n| *n += 1);
        writes[10].update(|n| *n += 1);
        writes[100].update(|n| *n += 1);

        assert_eq!(acc.get(), 499503);
        assert_eq!(memo.get(), 499503);
    })
    .dispose()
}

pub fn leptos_create_and_dispose_1000_scopes(runtime: RuntimeId) {
    let acc = Rc::new(Cell::new(0));
    let disposers = (0..1000)
        .map(|_| {
            create_scope(runtime, {
                let acc = Rc::clone(&acc);
                move |sc| {
                    let (r, w) = create_signal(sc, 0);
                    create_isomorphic_effect(sc, {
                        move |_| {
                            acc.set(r.get());
                        }
                    });
                    w.update(|n| *n += 1);
                }
            })
        })
        .collect::<Vec<_>>();
    for disposer in disposers {
        disposer.dispose();
    }
}
