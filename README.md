<!-- 
Please don't edit. This document has been generated from src/readme.tpl.md
--> 
# X-Path

- [X-Path](#x-path)
- [Features](#features)
- [Example](#example)
- [Cargo features](#cargo-features)
- [Evolutions](#evolutions)
- [Benchmarks](#benchmarks)
    - [Performance](#performance)
    - [Memory use](#memory-use)
- [A personal note & the future of reactive-signals](#a-personal-note-&-the-future-of-reactive-signals)


reactive-signals is a dx-first scope-based finegrained reactive system. It is based on the excellent ideas in
[leptos_reactive](https://crates.io/crates/leptos_reactive) but is written from scratch in order to
provide the simplest API and mental model possible for developers.

> This documentation assumes that you know [Leptos](https://crates.io/crates/leptos) and are familiar
> with the concepts of reactivity.
> - <sup>TBD</sup> (to be done) means that the feature will be added in the future.
> - <sup>TBC</sup> (to be confirmed) means that it is a possible future addition
>
> Note: This project is not yet ready for use! It needs a full test coverage first.


# Features

- Slim and powerful API surface. Essentially: Scope, Signal, signal!.
- Developer experience: You create reactive signals and they update automatically in a predictable manner.
  There's not much more to know.
- Memory and performance overhead that is so low that a developer doesn't need to worry about it.
- An easy-to-use [signal!] macro for creating all kinds of signals including data and functional signals,
  server-side and client-side signals etc.
- [Signal]s produce a reactive value, for data signals, it's the inner data and for functional signals,
  it's the value produced by the function. Subscribers are notified when the value is updated,
  or for a value that implements [PartialEq], when it is changed.
- Type-safe attached data to scopes. See the [Scope] doc.<sup>TBD</sup>
- 4 times less memory overhead and 3.5 times faster (worst case) than [leptos_reactive](https://crates.io/crates/leptos_reactive).
  See [Benchmarks](Self#Benchmarks) below.
- Push-pull updates: Guarantees that the nodes are only updated once and only if necessary.
  See the end of the [reactively](https://github.com/modderme123/reactively) readme for more information.<sup>TBC</sup>
- Tokio [tracing](https://crates.io/crates/tracing) compatibility.<sup>TBC</sup>
- async signals with runtimes using a custom async runtime when running in a web browser and
  [tokio](https://crates.io/crates/tokio) when running in a server. See the [signal!] doc.<sup>TBC</sup>
- Mirror the leptos_reactive API with deprecations that give instructions on how to upgrade
  to give a smooth upgrade experience. If there's interest, of course.<sup>TBC</sup>
- Production-class test-coverage.<sup>TBC</sup>
- See [Evolutions](Self#Evolutions) for more possible features.


# Example

```rust ignore
use reactive_signals::{signal, Scope, Scoped, runtimes::ClientRuntime};

enum UserActions {
    None,
    StartClicked,
    PlusClicked,
    MinusClicked,
    TextChanged(String),
}

// Let's do a simple "select your fruit and quantity in 30 seconds" app.

// The entire app needs to know which fruit and the quantity
#[derive(Default, Scoped)]
struct FruitSelection((Option<String>, usize));

// create a root scope.
let sc = ClientRuntime::new_root_scope();

// let's attach the FruitSelection to the scope
let sc = FruitSelection((String, 0)).attach_to(sc);



// Run the app.
fruit_list_app(sc);

fn fruit_list_app<RT: Runtime>(
    sc: FruitSelectionScope<RT>,
    actions: Signal<Data<UserActions>, RT>,
    displayed: Signal<EqData<String>, RT>) {

  let secs_remaning = signal!(sc, 30);
  let count = signal!(sc, 0);
  let name = signal!(sc, Option::<String>::None);
  let fruit_history = signal!(sc, Vec::<String>::new())
  
  let running = signal!(sc, false);

  
  signal!(sc, move || {
    match actions.get() {
        UserActions::StartClicked if !running.get() => {
            running.set(true);
            signal!(sc, 1 sec, async move |&mut interval| {
                if secs_remaining.get() <= 0 {
                    running.set(false);
                    interval = None; // stop running
                    int_sc.dispose(); // one of the few times you'll need to dispose a scope manually
                } else {
                    secs_remaning.update(|s| *s += 1);
                }
            });
        }
        UserActions::PlusClicked => count.update(|c| *c += 1),
        UserActions::MinusClicked if count.get() > 0 => count.update(|c| *c -= 1),
        UserActions::TextChanged(txt) => name.set(txt),
        _ => {}
    }
  });
}
```

# Cargo features

- `unsafe-cell`: Internally, the reactive-signals use [RefCell](https://doc.rust-lang.org/stable/core/cell/struct.RefCell.html) for interior mutability.
  Once reactive-signals is mature and if your app is well tested, then [UnsafeCell](https://doc.rust-lang.org/stable/core/cell/struct.UnsafeCell.html)
  can be used, resulting in a performance improvement of around 40% and a reduction in memory use by some 20%.


# Evolutions

- **Timetravel**. Due to how reactive-signals is structured it is possible to create state snapshots that can
  be used to create a real-time visualization of the signals, grouped by their
  scope with edges between connected signals. Each outside action or event would trigger
  a new state snapshot. A state snapshot would be visualized by highlighting the
  triggering signal and all its dependencies recursively. When the signal's value implements Debug or Display
  it can be used to visualize its content.
- **Polled signals**. Option to register signals for polling so that a runtime vec will contain all changed signals
  since the last polling. This could be used to group DOM updates into one update per frame avoiding
  the overhead of many small and costly calls out of the WASM. The usefulness of it for Leptos would need to be investigated.
- **Remote shim** (speculative). Everything that goes in or out of a WASM is converted between Rust and JS data.
  It should be possible to put a shim in that serializes it to a remote app. Why do such a thing? It would help to make
  full hot-reloading possible and to apply various tricks for greatly speeding up the compile times.


# Benchmarks

Measurements have been rounded for ease of reading and reasoning. They measure ScopeInner and SignalInner
(not part of public API) which are where the data is stored as Scope and Signal only has index (integer) data


## Performance

These measurements have been produced using [criterion](https://crates.io/crates/criterion) by measuring on
1000 instances and calculating the time for one. It has been measured on a Macbook M1.

| What                 | Time  | With `unsafe-cell`
| ---                  | ---   | ---
| Create a ScopeInner  | 10 ns |  8 ns
| Create a SignalInner | 55 ns | 50 ns
| Notify a subscriber  | 25 ns | 15 ns

The leptos_reactive profiling example "Leptos create 1000 signals" measures 245 µs.
The same measures 70 µs using reactive-signals. That makes for a 3.5 times improvement.

## Memory use

These measurements has been produced using [dhat](https://crates.io/crates/dhat) by creating
1000 instances and calculating the size of one.

| What                     | Heap use | With `unsafe-cell`
| ---                      | ---      | ---
| ScopeInner               | 40 bytes | 32 bytes
| SignalInner              | 80 bytes | 70 bytes
| Subscription<sup>*</sup> | 8 bytes  | 12 bytes

<sup>*</sup> The memory use for each signal subscription.

In leptos_reactive, 1000 signals and one memo uses 400kb and
in reactive-signals creating 1000 function signals each with a subscription
uses 100kb. In other words, reactive-signals use 4 times less memory than
leptos_reactive

Please see the benches, examples and tests for full details.


# A personal note & the future of reactive-signals

I have spent a lot of time on reactive-signals which have been entirely self-funded. Unfortunately,
I cannot continue like that (I would love to, though!).

The future of reactive-signals depends on you and if you want to fund the features listed with a <sup>TBC</sup>.

I have created a [fundraiser](https://opencollective.com/human-solutions/projects/reactive-signals) for it.

I'm open to any type of freelance contract work that would allow me to continue
developing and maintaining the open-source projects I have and plan to do.
See my [services](https://human.solutions/services/).

See my other [open-source projects](https://human.solutions/opensource/).

Feel free to reach out if you are interested!

