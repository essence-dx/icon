use clap::{Parser, Subcommand};
use dx_icon::engine::IconSearchEngine;
use dx_icon::index::IconIndex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static ENGINE: Mutex<Option<IconSearchEngine>> = Mutex::new(None);

fn push_unique(paths: &mut Vec<PathBuf>, path: PathBuf) {
    if !paths.iter().any(|existing| existing == &path) {
        paths.push(path);
    }
}

fn dx_icon_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Ok(root) = std::env::var("DX_ICON_ROOT") {
        push_unique(&mut roots, PathBuf::from(root));
    }

    if let Ok(home) = std::env::var("DX_HOME") {
        push_unique(&mut roots, PathBuf::from(home).join("icon"));
    }

    for start in current_search_starts() {
        for ancestor in start.ancestors() {
            let icon_child = ancestor.join("icon");
            if icon_child.join("data").is_dir() || icon_child.join("index").is_dir() {
                push_unique(&mut roots, icon_child);
            }
            if icon_crate_marker(ancestor) {
                push_unique(&mut roots, ancestor.to_path_buf());
            }
        }
    }

    let g_drive = PathBuf::from(r"G:\Dx\icon");
    if g_drive.join("data").is_dir() || g_drive.join("index").is_dir() {
        push_unique(&mut roots, g_drive);
    }

    roots
}

fn current_search_starts() -> Vec<PathBuf> {
    let mut starts = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            starts.push(parent.to_path_buf());
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        starts.push(cwd);
    }
    starts
}

fn icon_crate_marker(path: &Path) -> bool {
    path.join("Cargo.toml").is_file() && path.join("src").join("bin").join("icon.rs").is_file()
}

/// DX Icon CLI - Professional icon search and management tool
#[derive(Parser)]
#[command(name = "icon")]
#[command(version = "1.0.0")]
#[command(about = "Search and export icons from 305,612+ icons across 229 packs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for icons by name or keyword
    #[command(alias = "s")]
    Search {
        /// Search query
        query: String,

        /// Maximum number of results to return
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Filter by icon pack
        #[arg(short, long)]
        pack: Option<String>,
    },

    /// Export icons as SVG files
    #[command(alias = "e")]
    Export {
        /// Search query
        query: String,

        /// Output directory path
        output: PathBuf,

        /// Maximum number of icons to export
        #[arg(short, long, default_value = "10")]
        limit: usize,

        /// Filter by icon pack
        #[arg(short, long)]
        pack: Option<String>,
    },

    /// Download specific icons by name:pack format
    #[command(alias = "d")]
    Download {
        /// Icon specifications in format: name:pack (e.g., home:lucide arrow:lucide)
        #[arg(required = true)]
        icons: Vec<String>,

        /// Output directory path
        #[arg(short, long, default_value = "./")]
        output: PathBuf,
    },

    /// List all available icon packs
    #[command(alias = "p")]
    Packs,

    /// Download company logo
    #[command(alias = "l")]
    Logo {
        /// Company domain (e.g., google.com, github.com, stripe.com)
        domain: String,

        /// Output file path (e.g., ./logos/google.png)
        output: PathBuf,

        /// Logo size: 32, 64, 128, 256 (pixels)
        #[arg(short, long, default_value = "128")]
        size: u32,
    },
}

fn main() {
    let config = match dx_icon::dx_config::IconDxConfig::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[ERROR] Config: {}", e);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("[ERROR] {}", e);
        std::process::exit(1);
    }

    if let Err(e) = config.write_sr("icon", &[("tool", "icon"), ("action", "run"), ("status", "ok")]) {
        eprintln!("[WARN] Failed to write .sr: {}", e);
    }
    if let Err(e) = config.write_global_sr("icon", &[("tool", "icon"), ("action", "run"), ("status", "ok")]) {
        eprintln!("[WARN] Failed to write global .sr: {}", e);
    }
    if let Some(status) = config.read_status("icon") {
        eprintln!("[icon] sr cache verified: {} entries", status.len());
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Search { query, limit, pack } => search_icons(&query, limit, pack.as_deref())?,

        Commands::Export {
            query,
            output,
            limit,
            pack,
        } => export_icons(&query, &output, limit, pack.as_deref())?,

        Commands::Download { icons, output } => {
            let icon_specs: Vec<&str> = icons.iter().map(|s| s.as_str()).collect();
            download_icons(&icon_specs, &output)?;
        }

        Commands::Packs => list_packs()?,

        Commands::Logo {
            domain,
            output,
            size,
        } => download_logo(&domain, &output, size)?,
    }

    Ok(())
}

fn load_engine() -> anyhow::Result<()> {
    let mut engine_lock = ENGINE.lock().unwrap();

    if engine_lock.is_none() {
        let mut possible_paths = vec![];

        if let Ok(env_path) = std::env::var("DX_ICON_INDEX") {
            push_unique(&mut possible_paths, PathBuf::from(env_path));
        }

        for root in dx_icon_roots() {
            push_unique(&mut possible_paths, root.join("index"));
        }

        if let Ok(exe) = std::env::current_exe() {
            if let Some(parent) = exe.parent() {
                push_unique(&mut possible_paths, parent.join("index"));
                push_unique(&mut possible_paths, parent.join("../share/dx-icons/index"));
            }
        }

        push_unique(&mut possible_paths, PathBuf::from("index"));
        push_unique(
            &mut possible_paths,
            PathBuf::from("crates/media/icon/index"),
        );

        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            let home_path = PathBuf::from(home);
            push_unique(&mut possible_paths, home_path.join(".dx/icon/index"));
        }

        let index_dir = possible_paths.iter().find(|p| p.exists()).ok_or_else(|| {
            anyhow::anyhow!(
                "Index not found. Searched locations:\n{}",
                possible_paths
                    .iter()
                    .map(|p| format!("  - {}", p.display()))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

        let index = IconIndex::load_mmap(index_dir)?;
        *engine_lock = Some(IconSearchEngine::from_index(index)?);
    }

    Ok(())
}

fn with_engine<F, R>(f: F) -> anyhow::Result<R>
where
    F: FnOnce(&IconSearchEngine) -> R,
{
    load_engine()?;
    let engine_lock = ENGINE.lock().unwrap();
    Ok(f(engine_lock.as_ref().unwrap()))
}

fn search_icons(query: &str, limit: usize, pack_filter: Option<&str>) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    let results = search_for_output(query, limit, pack_filter)?;

    let elapsed = start.elapsed();

    if results.is_empty() {
        println!("[INFO] No results found for '{}'", query);
        return Ok(());
    }

    println!(
        "[SUCCESS] Found {} results ({:.2}s)\n",
        results.len(),
        elapsed.as_secs_f64()
    );

    for (i, result) in results.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, result.icon.name, result.icon.pack);
    }

    Ok(())
}

fn export_icons(
    query: &str,
    output_dir: &PathBuf,
    limit: usize,
    pack_filter: Option<&str>,
) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    let results = search_for_output(query, limit, pack_filter)?;

    if results.is_empty() {
        println!("[INFO] No icons found");
        return Ok(());
    }

    fs::create_dir_all(output_dir)?;

    for result in &results {
        let filename = format!("{}_{}.svg", result.icon.pack, result.icon.name);
        let filepath = output_dir.join(&filename);
        let svg_content = generate_svg(&result.icon.name, &result.icon.pack)?;
        fs::write(&filepath, svg_content)?;
        println!("[OK] {}", filename);
    }

    println!(
        "\n[SUCCESS] Exported {} icons ({:.2}s)",
        results.len(),
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

fn search_for_output(
    query: &str,
    limit: usize,
    pack_filter: Option<&str>,
) -> anyhow::Result<Vec<dx_icon::search::SearchResult>> {
    let candidate_limit = if pack_filter.is_some() {
        limit.saturating_mul(250).max(1000)
    } else {
        limit.saturating_mul(10)
    };
    let mut results = with_engine(|engine| engine.search(query, candidate_limit))?;

    if let Some(pack) = pack_filter {
        results.retain(|r| r.icon.pack == pack);
    }
    results.truncate(limit);

    Ok(results)
}

fn download_icons(icon_specs: &[&str], output_dir: &PathBuf) -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    fs::create_dir_all(output_dir)?;

    let mut downloaded = 0;
    let mut failed = 0;

    for spec in icon_specs {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts.len() != 2 {
            eprintln!("[ERROR] Invalid format: {} (use name:pack)", spec);
            failed += 1;
            continue;
        }

        let (name, pack) = (parts[0], parts[1]);

        match generate_svg(name, pack) {
            Ok(svg_content) => {
                let filename = format!("{}_{}.svg", pack, name);
                let filepath = output_dir.join(&filename);
                match fs::write(&filepath, svg_content) {
                    Ok(_) => {
                        println!("[OK] {}", filename);
                        downloaded += 1;
                    }
                    Err(e) => {
                        eprintln!("[ERROR] Failed to write {}: {}", filename, e);
                        failed += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("[ERROR] Icon {}:{} not found: {}", name, pack, e);
                failed += 1;
            }
        }
    }

    if downloaded > 0 {
        println!(
            "\n[SUCCESS] Downloaded {} icons ({:.2}s)",
            downloaded,
            start.elapsed().as_secs_f64()
        );
    }
    if failed > 0 {
        eprintln!("[WARNING] Failed: {}", failed);
    }

    Ok(())
}

fn list_packs() -> anyhow::Result<()> {
    let results = with_engine(|engine| engine.search("a", 10000))?;

    let mut packs: Vec<String> = results
        .iter()
        .map(|r| r.icon.pack.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    packs.sort();

    println!("[INFO] Available icon packs: {}\n", packs.len());
    for pack in packs {
        println!("  - {}", pack);
    }

    Ok(())
}

fn download_logo(domain: &str, output: &PathBuf, size: u32) -> anyhow::Result<()> {
    println!(
        "[INFO] Downloading logo for '{}' from Hunter.io (16M+ logos)...",
        domain
    );

    // Validate size
    if ![32, 64, 128, 256].contains(&size) {
        return Err(anyhow::anyhow!(
            "Invalid size: {}. Use: 32, 64, 128, or 256",
            size
        ));
    }

    // Hunter.io Logo API - 100% free, no API key needed, 16M+ logos
    // Format: https://logos.hunter.io/{domain}
    // Note: Size parameter may not be supported, returns default size
    let url = format!("https://logos.hunter.io/{}", domain);

    println!("[INFO] Fetching from: {}", url);

    let response = reqwest::blocking::get(&url)?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download logo: HTTP {} - Domain '{}' may not have a logo available",
            response.status(),
            domain
        ));
    }

    let content = response.bytes()?;

    // Check if response is actually an error page (HTML)
    if content.starts_with(b"<!DOCTYPE") || content.starts_with(b"<html") {
        return Err(anyhow::anyhow!(
            "Logo for '{}' not found. Make sure to use the full domain (e.g., google.com, not just google)",
            domain
        ));
    }

    // Check if it's a valid image (PNG, JPEG, SVG, or WebP)
    let is_valid_image = content.starts_with(b"\x89PNG")  // PNG
        || content.starts_with(b"\xFF\xD8\xFF")  // JPEG
        || content.starts_with(b"<svg")  // SVG
        || content.starts_with(b"RIFF"); // WebP

    if !is_valid_image {
        return Err(anyhow::anyhow!(
            "Invalid image received for domain '{}'. Logo may not be available.",
            domain
        ));
    }

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(output, &content)?;

    // Detect image type
    let image_type = if content.starts_with(b"\x89PNG") {
        "PNG"
    } else if content.starts_with(b"\xFF\xD8\xFF") {
        "JPEG"
    } else if content.starts_with(b"<svg") {
        "SVG"
    } else if content.starts_with(b"RIFF") {
        "WebP"
    } else {
        "Unknown"
    };

    println!(
        "[SUCCESS] Logo saved to: {} ({} bytes, {} format)",
        output.display(),
        content.len(),
        image_type
    );

    Ok(())
}

fn generate_svg(name: &str, pack: &str) -> anyhow::Result<String> {
    let mut possible_data_dirs = vec![];

    if let Ok(env_path) = std::env::var("DX_ICON_DATA") {
        push_unique(&mut possible_data_dirs, PathBuf::from(env_path));
    }

    for root in dx_icon_roots() {
        push_unique(&mut possible_data_dirs, root.join("data"));
    }

    if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
        let home_path = PathBuf::from(home);
        push_unique(&mut possible_data_dirs, home_path.join(".dx/icon/data"));
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            push_unique(&mut possible_data_dirs, parent.join("data"));
            push_unique(
                &mut possible_data_dirs,
                parent.join("../share/dx-icons/data"),
            );
            push_unique(
                &mut possible_data_dirs,
                parent.join("../../crates/media/icon/data"),
            );
        }
    }

    push_unique(
        &mut possible_data_dirs,
        PathBuf::from("crates/media/icon/data"),
    );
    push_unique(&mut possible_data_dirs, PathBuf::from("data"));
    push_unique(&mut possible_data_dirs, PathBuf::from("../../data"));

    let data_dir = possible_data_dirs
        .iter()
        .find(|p| p.exists())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Data directory not found. Searched locations:\n{}",
                possible_data_dirs
                    .iter()
                    .map(|p| format!("  - {}", p.display()))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

    let pack_file = data_dir.join(format!("{}.json", pack));

    if !pack_file.exists() {
        return Err(anyhow::anyhow!("Pack '{}' not found", pack));
    }

    let content = fs::read_to_string(&pack_file)?;
    let pack_data: serde_json::Value = serde_json::from_str(&content)?;

    let icon_data = pack_data["icons"]
        .get(name)
        .ok_or_else(|| anyhow::anyhow!("Icon '{}' not found in pack '{}'", name, pack))?;

    let body = icon_data["body"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Icon body not found"))?;

    let width = icon_data["width"]
        .as_f64()
        .or_else(|| pack_data["width"].as_f64())
        .unwrap_or(24.0);

    let height = icon_data["height"]
        .as_f64()
        .or_else(|| pack_data["height"].as_f64())
        .unwrap_or(24.0);

    Ok(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">{}</svg>"#,
        width, height, width, height, body
    ))
}
