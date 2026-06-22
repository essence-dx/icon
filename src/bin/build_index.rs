use clap::Parser;
use dx_icon::builder::IndexBuilder;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "build_index")]
#[command(about = "Build the DX icon search index from G-drive icon pack data")]
struct Args {
    /// Icon pack JSON directory. Defaults to DX_ICON_DATA or G:\Dx\icon\data.
    #[arg(long)]
    data: Option<PathBuf>,

    /// Output index directory. Defaults to DX_ICON_INDEX or G:\Dx\icon\index.
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let icon_root = discover_icon_root();
    let data_dir = args
        .data
        .or_else(|| std::env::var_os("DX_ICON_DATA").map(PathBuf::from))
        .unwrap_or_else(|| icon_root.join("data"));
    let output_dir = args
        .output
        .or_else(|| std::env::var_os("DX_ICON_INDEX").map(PathBuf::from))
        .unwrap_or_else(|| icon_root.join("index"));

    IndexBuilder::build_from_dir(&data_dir, &output_dir)?;

    Ok(())
}

fn discover_icon_root() -> PathBuf {
    if let Some(root) = std::env::var_os("DX_ICON_ROOT").map(PathBuf::from) {
        return root;
    }

    if let Some(root) = std::env::var_os("DX_HOME")
        .map(PathBuf::from)
        .map(|home| home.join("icon"))
        .filter(|path| path.join("data").is_dir())
    {
        return root;
    }

    for start in current_search_starts() {
        for ancestor in start.ancestors() {
            let candidate = ancestor.join("icon");
            if candidate.join("data").is_dir() {
                return candidate;
            }
            if ancestor.join("data").is_dir() && icon_crate_marker(ancestor) {
                return ancestor.to_path_buf();
            }
        }
    }

    let g_drive = PathBuf::from(r"G:\Dx\icon");
    if g_drive.join("data").is_dir() {
        return g_drive;
    }

    PathBuf::from(".")
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
