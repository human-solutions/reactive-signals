//!
//! Run with:
//!  - cargo run --example heap_profiling --features=profile --profile=heap
//!
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    ///// 1000 Scopes and Signals

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_nested_scopes_each_with_a_signal();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 409_720);
        assert_eq!(stats.max_blocks, 3_006);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 457_968);
        assert_eq!(stats.max_blocks, 3_006);
    }
    drop(profile);

    ///// 1000 Scopes

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_nested_scopes();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 33_336);
        assert_eq!(stats.max_blocks, 2);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 41_536);
        assert_eq!(stats.max_blocks, 2);
    }
    drop(profile);

    ///// 1000 Data Signals

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_data_signals();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 98_842);
        assert_eq!(stats.max_blocks, 1004);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 115_074);
        assert_eq!(stats.max_blocks, 1004);
    }
    drop(profile);

    ///// 1000 Func Signals

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_func_signals();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 94_838);
        assert_eq!(stats.max_blocks, 1003);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 115_058);
        assert_eq!(stats.max_blocks, 1003);
    }
    drop(profile);

    ///// 1000 Func Signals with dep

    let stats_no_deps = stats.clone();

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_func_signals_with_one_subscription();

    let stats = dhat::HeapStats::get();

    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 131_210);
        assert_eq!(stats.max_bytes - stats_no_deps.max_bytes, 36_372);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 147_442);
        assert_eq!(stats.max_bytes - stats_no_deps.max_bytes, 32_384);
    }
    drop(profile);
}
