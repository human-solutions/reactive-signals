#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let _profiler = dhat::Profiler::builder().testing().build();

    reactor::tests::profile::create_1000_nested();

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
}
