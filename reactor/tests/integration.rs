use reactor::{runtimes::ClientRuntime, signal};

#[test]
fn test_use() {
    let sx = ClientRuntime::new_root_scope();

    let count = signal!(sx, 5);
    let even = signal!(sx, 2);

    let _is_even = count.with(|val| *val % 2 == 0);

    signal!(sx, move || {
        let is_even = count.with(|v| *v % 2 == 0);
        if is_even {
            even.set(count.get())
        }
    });

    assert_eq!(count.get(), 5);
    assert_eq!(even.get(), 2);

    count.set(6);
    assert_eq!(count.get(), 6);
    assert_eq!(even.get(), 6);

    let _string_sig = signal!(sx, "hi".to_string());
}
