#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use leptos::create_runtime;

// Run with:
// cargo run --example leptos_heap --features=profile --profile=heap

fn main() {
    let _profiler = dhat::Profiler::builder().testing().build();

    let runtime = create_runtime();
    benchmarks::leptos_create_1000_signals(runtime);

    let stats = dhat::HeapStats::get();
    assert_eq!(stats.max_bytes, 453_712);
    assert_eq!(stats.max_blocks, 3_016);
}
