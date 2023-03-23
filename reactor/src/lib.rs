//!
//! Reactor is a dx-first scope-based finegrained reactive system. It is based on the excellent ideas in
//! [leptos_reactive](https://crates.io/crates/leptos_reactive) but is written from scratch in order to
//! provide the simplest API and mental model possible for developers.
//!
//! This documentation assumes that you know [Leptos](https://crates.io/crates/leptos) and are familiar with the concepts of reactivity.
//! - <sup>TBD</sup> (to be done) means that the feature will be added in the future.
//! - <sup>TBC</sup> (to be confirmed) means that it is a possible future addition
//!
//! Note: This project is not yet ready for use! It needs a full test coverage first.
//!
//! # Features
//!
//! - Extremely slim API surface that is really powerful.
//! - Developer experience: You create reactive signals and they update automatically in a predictable manner. There's not much more to know.
//! - Memory and performance overhead so low that a developer doesn't need to worry about it.
//! - An easy-to-use [signal!] macro for creating all kinds of signals including data and functional signals, _server_ and _client_ only signals etc.
//! - [Signal]s produce a reactive value, for data signals it's the contained data and for functional signals it's the value produce by the function.
//!   When the value implements [PartialEq] then the subscribers will be notified only if the value changes.
//! - Type-safe attached data to scopes. See the [Scope] doc.<sup>TBD</sup>
//! - 4 times less memory overhead and 3.5 times faster (worst case) than [leptos_reactive](https://crates.io/crates/leptos_reactive). See [Benchmarks](Self#Benchmarks) below.
//! - Production-class test-coverage<sup>TBC</sup>
//! - Mirror the leptos_reactive api with deprecations that gives instructions on how to upgrade in order
//!   to give a really smooth upgrade experience. If there's interest, of course.<sup>TBC</sup>
//! - Push-pull updates: Guarantees that the nodes are only updated once and only if necessary.
//!   See end of the [reactively](https://github.com/modderme123/reactively) readme for more information.<sup>TBC</sup>
//! - Tokio [tracing](https://crates.io/crates/tracing) compatibility.<sup>TBC</sup>
//! - async signals with runtimes using a custom async runtime when running in a web browser and
//!   [tokio](https://crates.io/crates/tokio) when running in a server. See the [signal!] doc.<sup>TBC</sup>
//! - See [Evolutions](Self#Evolutions) for more possible features.
//!
//!
//! # Example
//!
//! ```rust ignore
//! use reactor::{signal, Scope, Scoped, runtimes::ClientRuntime};
//!
//! // Let's do a simple fruit list app
//!
//! // The entire app needs the fruit list
//! #[derive(Default, Scoped)]
//! struct FruitList(Vec<(String, usize)>);
//! # struct FruitListScope(FruitList);
//! # impl FruitList {
//! #   fn attach_to(self, sc: Scope<ClientRuntime>) -> FruitListScope { todo!() }
//! # }
//!
//! // create a root scope.
//! let sc = ClientRuntime::new_root_scope();
//!
//! // let's attach the FruitList to the scope
//! let sc = FruitList::default().attach_to(sc);
//!
//!
//!
//! // run the app. This would normally be a Leptos component that
//! // produces html output.
//! fruit_list_app(sc);
//!
//! fn fruit_list_app(sc: FruitListScope) {
//!   let fruit_history = signal!(sc, Vec::<String>::new())
//!   let current_fruit = signal!(sc, String::new());
//!   let current_count = signal!(sc, 0);
//!   
//!   signal!(sc, move || {
//!     
//!   });
//! }
//! ```
//!
//! # Cargo features
//!
//! - `unsafe-cell`: Internally, reactor uses [RefCell](::core::cell::RefCell) for interior mutability. Once reactor is mature and if your app
//!   is well tested, then [UnsafeCell](::core::cell::UnsafeCell) can be used, resulting in a performance improvement of around 40% and a reduction
//!   in memory use by some 20%.
//!
//!
//! # Evolutions
//!
//! - **Timetravel**. Due to how Reactor is structured it is possible to create state snapshots that can
//!   be used to create a realtime visualization of the signals, grouped by their
//!   scope with edges between connected signals. Each outside action or event would trigger
//!   a new state snapshot. A state snapshot would be visualized by highlightning the
//!   triggering signal and all it's dependencies recursively. When the signal's value implements Debug or Display
//!   it can be used to visualize it's content.
//! - **Polled signals**. Option to register signals for polling so that a runtime vec will contain all changed signals
//!   since last polling. This could be used
//!   to group DOM updates into one update per frame avoiding the overhead of many small and costly calls out of the WASM.
//!   Usefulness for Leptos would need to be investigated.
//! - **Remote shim** (speculative). Everything that goes in or out of a WASM is converted between Rust and JS style data.
//!   It should be possible to put a shim in that serializes it to a remote app. Why do such a thing? It would help making
//!   full hot-reloading possible and to apply various tricks for greatly speeding up the compile times.
//!
//!
//! # Benchmarks
//!
//! Measurements have been rounded for ease of reading and reasoning. They measure ScopeInner and SignalInner
//! (not part of public api) which are where the data is stored as Scope and Signal only has index (integer) data
//!
//!
//! ## Performance
//!
//! These measurements has been produced using [criterion](https://crates.io/crates/criterion) by measuring on
//! 1000 instances and calculating the time for one. It has been measured on a Macbook M1.
//!
//! | What                 | Time  | With `unsafe-cell`
//! | ---                  | ---   | ---
//! | Create a ScopeInner  | 10 ns |  8 ns
//! | Create a SignalInner | 55 ns | 50 ns
//! | Notify a subscriber  | 25 ns | 15 ns
//!
//! The leptos_reactive profiling example "Leptos create 1000 signals" measures 245 µs.
//! The same measures 70 µs using Reactor. That makes for a 3.5 times improvement.
//!
//! ## Memory use
//!
//! These measurements has been produced using [dhat](https://crates.io/crates/dhat) by creating
//! 1000 instances and calculating the size of one.
//!
//! | What                     | Heap use | With `unsafe-cell`
//! | ---                      | ---      | ---
//! | ScopeInner               | 40 bytes | 32 bytes
//! | SignalInner              | 80 bytes | 70 bytes
//! | Subscription<sup>*</sup> | 8 bytes  | 12 bytes
//!
//! <sup>*</sup> The memory use for each signal subscription.
//!
//! In leptos_reactive, 1000 signals and one memo uses 400kb and
//! in Reactor creating 1000 function signals each with a subscription
//! uses 100kb. In other words Reactor uses 4 times less memory than
//! leptos_reactive
//!
//! Please see the benches, examples and tests for full details.
//!
//! # A personal note & the future of Reactor
//!
//! I have spent a lot of time on Reactor which has been entirely self-funded. Unfortunately,
//! it is not possible for me to continue like that (I would love to!).
//!
//! The future of Reactor depends on your reactions and if you want to fund the really
//! cool features, the ones I listed with a <sup>TBC</sup>. I believe there's a huge potiential for
//! Leptos and Reactor together.
//!
//! Reactor could also be used outside of Leptos, but for now, since my heart lies
//! with Leptos, I won't put effort in making Reactor known.
//!
//! Also, I'm a seasoned (read 50 years old) developer with a lot of Team Leading
//! and mentoring experience as well as my Rust & Swift development skills. I'm open
//! to any type of freelance contract work that would allow me to continue
//! developing and maintaining the open source projects I have and plan to do.
//!
//! Mentioning my other open source projects, there's a couple that I have been working
//! on for a while in stealth mode:
//! 1. an easy way to package Rust libraries as iOS and Android packages, including resources.
//! 1. packaging Leptos apps into iOS and Android packages that can be used as a simple WebView
//! from the native code with autogenerated hooks for a Web - Swift/Kotlin - Rust integration.
//!
//! Feel free to reach out if you are interested!
//!

#[cfg(any(test, feature = "profile"))]
pub mod tests;

mod iter;
mod macros;
mod primitives;
pub mod runtimes;
mod scope;
mod signals;

pub use scope::Scope;
#[doc(hidden)]
pub use signals::kinds::*;
pub use signals::Signal;

use runtimes::Runtime;
use scope::ScopeInner;
#[doc(hidden)]
pub use signals::types;
use std::cell;

#[cfg(not(feature = "unsafe-cell"))]
type CellType<T> = cell::RefCell<T>;
#[cfg(feature = "unsafe-cell")]
type CellType<T> = cell::UnsafeCell<T>;
