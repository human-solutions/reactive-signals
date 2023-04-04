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
        assert_eq!(stats.max_bytes, 301_044);
        assert_eq!(stats.max_blocks, 3_005);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 349_284);
        assert_eq!(stats.max_blocks, 3_005);
    }
    drop(profile);

    ///// 1000 Scopes

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_nested_scopes();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 32_768);
        assert_eq!(stats.max_blocks, 1);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 40_960);
        assert_eq!(stats.max_blocks, 1);
    }
    drop(profile);

    ///// 1000 Data Signals

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_data_signals();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 73_664);
        assert_eq!(stats.max_blocks, 1002);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 89_888);
        assert_eq!(stats.max_blocks, 1002);
    }
    drop(profile);

    ///// 1000 Func Signals

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_func_signals();

    let stats = dhat::HeapStats::get();
    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 69_664);
        assert_eq!(stats.max_blocks, 1002);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 89_888);
        assert_eq!(stats.max_blocks, 1002);
    }
    drop(profile);

    ///// 1000 Func Signals with dep

    let stats_no_deps = stats.clone();

    let profile = dhat::Profiler::builder().testing().build();

    reactive_signals::tests::profile::create_1000_func_signals_with_one_subscription();

    let stats = dhat::HeapStats::get();

    #[cfg(feature = "unsafe-cell")]
    {
        assert_eq!(stats.max_bytes, 81_768);
        assert_eq!(stats.max_bytes - stats_no_deps.max_bytes, 12_104);
    }
    #[cfg(not(feature = "unsafe-cell"))]
    {
        assert_eq!(stats.max_bytes, 98_000);
        assert_eq!(stats.max_bytes - stats_no_deps.max_bytes, 8_112);
    }
    drop(profile);
}
