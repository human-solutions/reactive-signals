use criterion::{criterion_group, criterion_main, Criterion};

fn rs_deep_update(b: &mut Criterion) {
    use reactive_signals::{signal, types::Func, Scope, Signal};

    b.bench_function("rs_deep_update", |b| {
        b.iter(|| {
            let (guard, sc) = Scope::new_client_side_root_scope();
            let signal = signal!(sc, 0);
            let mut memos = Vec::<Signal<Func<i32>>>::new();
            for i in 0..1000usize {
                let prev = memos.get(i.saturating_sub(1)).copied();
                if let Some(prev) = prev {
                    memos.push(signal!(sc, move || prev.get() + 1))
                } else {
                    memos.push(signal!(sc, move || signal.get() + 1))
                }
            }
            signal.set(1);
            assert_eq!(memos[999].get(), 1001);
            guard
        })
    });
}

criterion_group!(benches, rs_deep_update,);

criterion_main!(benches);
