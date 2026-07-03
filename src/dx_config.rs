use std::collections::HashMap;
use anyhow::Context;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct IconDxConfig {
    pub workspace_root: PathBuf,
    pub sr_dir: PathBuf,
    pub receipts_dir: PathBuf,
}

impl IconDxConfig {
    pub fn load() -> anyhow::Result<Self> {
        let workspace_root = discover_workspace_root()?;
        let sr_dir = workspace_root.join("sr");
        let receipts_dir = workspace_root.join("receipts");

        std::fs::create_dir_all(&sr_dir)
            .with_context(|| format!("Failed to create sr_dir: {}", sr_dir.display()))?;
        std::fs::create_dir_all(&receipts_dir)
            .with_context(|| format!("Failed to create receipts_dir: {}", receipts_dir.display()))?;

        Ok(Self {
            workspace_root,
            sr_dir,
            receipts_dir,
        })
    }

    pub fn sr_path(&self, name: &str) -> PathBuf {
        self.sr_dir.join(format!("{}.sr", name))
    }

    pub fn write_sr(&self, name: &str, entries: &[(&str, &str)]) -> std::io::Result<()> {
        let path = self.sr_path(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut buf: Vec<u8> = Vec::new();
        for (key, value) in entries {
            write!(buf, "{key}=")?;
            Self::write_llm_value(&mut buf, value)?;
            buf.push(b'\n');
        }
        let tmp = path.with_extension("sr.tmp");
        std::fs::write(&tmp, &buf)?;
        std::fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn read_status(&self, name: &str) -> Option<HashMap<String, String>> {
        let sr_path = self.sr_path(name);
        let (doc, _from_machine) = serializer::try_read_machine_or_sr(&sr_path)?;
        let mut map = HashMap::new();
        for (key, value) in &doc.context {
            map.insert(key.clone(), value.to_string());
        }
        Some(map)
    }

    pub fn machine_path(&self, name: &str) -> PathBuf {
        self.sr_dir.join(format!("{}.machine", name))
    }

    fn write_llm_value(buf: &mut Vec<u8>, value: &str) -> std::io::Result<()> {
        if value.is_empty() {
            buf.extend_from_slice(b"\"\"");
            return Ok(());
        }
        let needs_quoting = value.contains(|c: char| {
            c.is_ascii_whitespace() || c == '"' || c == '[' || c == ']' || c == '=' || c == '#'
        });
        if needs_quoting {
            buf.push(b'"');
            for c in value.chars() {
                if c == '"' || c == '\\' { buf.push(b'\\'); }
                let mut tmp = [0u8; 4];
                buf.extend_from_slice(c.encode_utf8(&mut tmp).as_bytes());
            }
            buf.push(b'"');
        } else {
            buf.extend_from_slice(value.as_bytes());
        }
        Ok(())
    }
}

fn discover_workspace_root() -> anyhow::Result<PathBuf> {
    if let Some(root) = std::env::var_os("DX_HOME").map(PathBuf::from) {
        let icon_dir = root.join("icon");
        if icon_dir.join("Cargo.toml").is_file() {
            return Ok(icon_dir);
        }
        return Ok(root);
    }

    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            if ancestor.join("Cargo.toml").is_file() && ancestor.join("src").join("bin").join("icon.rs").is_file() {
                return Ok(ancestor.to_path_buf());
            }
        }
    }

    let g_drive = PathBuf::from(r"G:\Dx\icon");
    if g_drive.join("Cargo.toml").is_file() {
        return Ok(g_drive);
    }

    anyhow::bail!("Cannot discover icon workspace root. Set DX_HOME or run from within the icon crate")
}
