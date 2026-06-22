use dx_icon::index::IconIndex;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let index_dir = PathBuf::from("index");

    println!("=== Memory-Mapped vs Standard Load Benchmark ===\n");
    println!("Testing with 305,612 icons, zstd compressed index\n");

    // Warm up the file system cache
    println!("Warming up file system cache...");
    let _ = IconIndex::load(&index_dir)?;
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Test 1: Standard load (std::fs::read)
    println!("\n--- Test 1: Standard Load (std::fs::read) ---");
    let mut total_time = std::time::Duration::ZERO;
    let iterations = 10;

    for i in 1..=iterations {
        let start = Instant::now();
        let index = IconIndex::load(&index_dir)?;
        let elapsed = start.elapsed();
        total_time += elapsed;

        println!(
            "  Run {}: {:.2}ms (FST: {} bytes, Meta: {} bytes)",
            i,
            elapsed.as_secs_f64() * 1000.0,
            index.fst_bytes.len(),
            index.metadata_bytes.len()
        );
    }

    let avg_standard = total_time / iterations;
    println!("\nAverage: {:.2}ms", avg_standard.as_secs_f64() * 1000.0);

    // Clear cache effect
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Test 2: Memory-mapped load
    println!("\n--- Test 2: Memory-Mapped Load (memmap2) ---");
    total_time = std::time::Duration::ZERO;

    for i in 1..=iterations {
        let start = Instant::now();
        let index = IconIndex::load_mmap(&index_dir)?;
        let elapsed = start.elapsed();
        total_time += elapsed;

        println!(
            "  Run {}: {:.2}ms (FST: {} bytes, Meta: {} bytes)",
            i,
            elapsed.as_secs_f64() * 1000.0,
            index.fst_bytes.len(),
            index.metadata_bytes.len()
        );
    }

    let avg_mmap = total_time / iterations;
    println!("\nAverage: {:.2}ms", avg_mmap.as_secs_f64() * 1000.0);

    // Comparison
    println!("\n=== Results ===");
    println!(
        "Standard Load:      {:.2}ms",
        avg_standard.as_secs_f64() * 1000.0
    );
    println!(
        "Memory-Mapped Load: {:.2}ms",
        avg_mmap.as_secs_f64() * 1000.0
    );

    if avg_mmap < avg_standard {
        let improvement = ((avg_standard.as_secs_f64() - avg_mmap.as_secs_f64())
            / avg_standard.as_secs_f64())
            * 100.0;
        println!("\n✅ Memory-mapped is FASTER by {:.1}%", improvement);
        println!("Recommendation: Use load_mmap()");
    } else {
        let slower = ((avg_mmap.as_secs_f64() - avg_standard.as_secs_f64())
            / avg_standard.as_secs_f64())
            * 100.0;
        println!("\n❌ Memory-mapped is SLOWER by {:.1}%", slower);
        println!("Recommendation: Use standard load()");
    }

    Ok(())
}
