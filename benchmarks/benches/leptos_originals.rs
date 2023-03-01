///
/// The original benchmarks from the Leptos repository.
///
use criterion::{criterion_group, criterion_main, Criterion};
use leptos::{
    create_isomorphic_effect, create_memo, create_runtime, create_scope, create_signal, SignalGet,
    SignalUpdate,
};
use std::{cell::Cell, rc::Rc};

pub fn leptos_create_1000_signals(c: &mut Criterion) {
    let runtime = create_runtime();
    c.bench_function("Leptos create 1000 signals", |b| {
        b.iter(|| {
            create_scope(runtime, |cx| {
                let _acc = Rc::new(Cell::new(0));
                let sigs = (0..1000).map(|n| create_signal(cx, n)).collect::<Vec<_>>();
                let reads = sigs.iter().map(|(r, _)| *r).collect::<Vec<_>>();
                let _writes = sigs.iter().map(|(_, w)| *w).collect::<Vec<_>>();
                let memo = create_memo(cx, move |_| reads.iter().map(|r| r.get()).sum::<i32>());
                assert_eq!(memo.get(), 499500);
            })
            .dispose()
        });
    });
}

fn leptos_create_and_update_1000_signals(c: &mut Criterion) {
    let runtime = create_runtime();
    c.bench_function("Leptos create and update 1000 signals", |b| {
        b.iter(|| {
            create_scope(runtime, |cx| {
                let acc = Rc::new(Cell::new(0));
                let sigs = (0..1000).map(|n| create_signal(cx, n)).collect::<Vec<_>>();
                let reads = sigs.iter().map(|(r, _)| *r).collect::<Vec<_>>();
                let writes = sigs.iter().map(|(_, w)| *w).collect::<Vec<_>>();
                let memo = create_memo(cx, move |_| reads.iter().map(|r| r.get()).sum::<i32>());
                assert_eq!(memo.get(), 499500);
                create_isomorphic_effect(cx, {
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
        });
    });
}

fn leptos_create_and_dispose_1000_scopes(c: &mut Criterion) {
    let runtime = create_runtime();
    c.bench_function("Leptos create and dispose 1000 scopes", |b| {
        b.iter(|| {
            let acc = Rc::new(Cell::new(0));
            let disposers = (0..1000)
                .map(|_| {
                    create_scope(runtime, {
                        let acc = Rc::clone(&acc);
                        move |cx| {
                            let (r, w) = create_signal(cx, 0);
                            create_isomorphic_effect(cx, {
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
        });
    });
}
criterion_group!(
    benches,
    leptos_create_1000_signals,
    leptos_create_and_update_1000_signals,
    leptos_create_and_dispose_1000_scopes,
);
criterion_main!(benches);
