/// Benchmark comparing LZ4 vs zstd compression for icon index
use dx_icon::index::IconIndex;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    println!("=== Compression Benchmark: LZ4 vs zstd ===\n");

    let index_dir = PathBuf::from("index");

    // Test LZ4 (current implementation)
    println!("📦 LZ4 Compression (Current):");

    let start = Instant::now();
    let index = IconIndex::load(&index_dir)?;
    let lz4_load_time = start.elapsed();

    println!("  Load time: {:?}", lz4_load_time);
    println!("  FST size: {} bytes", index.fst_bytes.len());
    println!("  Metadata size: {} bytes", index.metadata_bytes.len());
    println!(
        "  Total uncompressed: {} MB",
        (index.fst_bytes.len() + index.metadata_bytes.len()) as f64 / 1_000_000.0
    );

    // Get compressed sizes
    let fst_compressed = std::fs::metadata(index_dir.join("index.fst.lz4"))?.len();
    let meta_compressed = std::fs::metadata(index_dir.join("index.meta.lz4"))?.len();

    println!(
        "  FST compressed: {} MB",
        fst_compressed as f64 / 1_000_000.0
    );
    println!(
        "  Metadata compressed: {} MB",
        meta_compressed as f64 / 1_000_000.0
    );
    println!(
        "  Total compressed: {} MB",
        (fst_compressed + meta_compressed) as f64 / 1_000_000.0
    );

    let lz4_ratio = (index.fst_bytes.len() + index.metadata_bytes.len()) as f64
        / (fst_compressed + meta_compressed) as f64;
    println!("  Compression ratio: {:.2}x", lz4_ratio);

    // Test zstd compression
    println!("\n📦 zstd Compression (Simulated):");

    let start = Instant::now();
    let zstd_fst = zstd::encode_all(&index.fst_bytes[..], 3)?;
    let zstd_meta = zstd::encode_all(&index.metadata_bytes[..], 3)?;
    let zstd_compress_time = start.elapsed();

    println!("  Compression time: {:?}", zstd_compress_time);
    println!(
        "  FST compressed: {} MB",
        zstd_fst.len() as f64 / 1_000_000.0
    );
    println!(
        "  Metadata compressed: {} MB",
        zstd_meta.len() as f64 / 1_000_000.0
    );
    println!(
        "  Total compressed: {} MB",
        (zstd_fst.len() + zstd_meta.len()) as f64 / 1_000_000.0
    );

    let zstd_ratio = (index.fst_bytes.len() + index.metadata_bytes.len()) as f64
        / (zstd_fst.len() + zstd_meta.len()) as f64;
    println!("  Compression ratio: {:.2}x", zstd_ratio);

    // Test zstd decompression
    let start = Instant::now();
    let _fst_decompressed = zstd::decode_all(&zstd_fst[..])?;
    let _meta_decompressed = zstd::decode_all(&zstd_meta[..])?;
    let zstd_decompress_time = start.elapsed();

    println!("  Decompression time: {:?}", zstd_decompress_time);

    // Comparison
    println!("\n=== Comparison ===");
    println!("Size Difference:");
    let size_diff =
        (fst_compressed + meta_compressed) as i64 - (zstd_fst.len() + zstd_meta.len()) as i64;
    let size_diff_pct = (size_diff as f64 / (fst_compressed + meta_compressed) as f64) * 100.0;
    println!(
        "  LZ4: {} MB",
        (fst_compressed + meta_compressed) as f64 / 1_000_000.0
    );
    println!(
        "  zstd: {} MB",
        (zstd_fst.len() + zstd_meta.len()) as f64 / 1_000_000.0
    );
    println!("  Savings: {} KB ({:.1}%)", size_diff / 1024, size_diff_pct);

    println!("\nSpeed Difference:");
    println!("  LZ4 load: {:?}", lz4_load_time);
    println!("  zstd load: {:?}", zstd_decompress_time);
    let speed_diff = zstd_decompress_time.as_micros() as f64 / lz4_load_time.as_micros() as f64;
    println!("  zstd is {:.2}x slower", speed_diff);

    println!("\nCompression Ratio:");
    println!("  LZ4: {:.2}x", lz4_ratio);
    println!("  zstd: {:.2}x", zstd_ratio);
    println!("  zstd is {:.2}x better", zstd_ratio / lz4_ratio);

    // Recommendation
    println!("\n=== Recommendation ===");
    if size_diff_pct > 20.0 && speed_diff < 2.0 {
        println!("✅ Consider switching to zstd");
        println!("   Significant space savings with acceptable speed penalty");
    } else if size_diff_pct < 10.0 {
        println!("✅ Keep LZ4");
        println!("   Minimal space savings, LZ4 is faster");
    } else {
        println!("⚖️  Trade-off decision");
        println!(
            "   {:.1}% space savings vs {:.2}x slower decompression",
            size_diff_pct, speed_diff
        );
    }

    Ok(())
}
