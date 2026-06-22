use crate::types::IconMetadata;
use anyhow::Result;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// Serialized icon index with FST and rkyv metadata
/// Optimized with zstd compression for compact storage (6 MB for 305K icons)
pub struct IconIndex {
    pub fst_bytes: Vec<u8>,
    pub metadata_bytes: Vec<u8>,
}

impl IconIndex {
    /// Build index from icon metadata
    pub fn build(icons: Vec<IconMetadata>) -> Result<Self> {
        // Build FST for name -> id mapping
        let mut builder = fst::MapBuilder::memory();
        let mut sorted_icons: Vec<_> = icons
            .iter()
            .enumerate()
            .map(|(idx, icon)| {
                // Create unique key: pack:name
                let key = format!("{}:{}", icon.pack, icon.name);
                (key, idx as u64)
            })
            .collect();
        sorted_icons.sort_by(|a, b| a.0.cmp(&b.0));

        for (name, id) in sorted_icons {
            builder.insert(name.as_bytes(), id)?;
        }

        let fst_bytes = builder.into_inner()?;

        // Serialize metadata with rkyv (zero-copy)
        let metadata_bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&icons)?.to_vec();

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }

    /// Save index to disk with zstd compression (35.5% smaller than LZ4)
    pub fn save(&self, path: &Path) -> Result<()> {
        let compressed_fst = zstd::encode_all(&self.fst_bytes[..], 3)?;
        let compressed_metadata = zstd::encode_all(&self.metadata_bytes[..], 3)?;

        std::fs::write(path.join("index.fst.zst"), compressed_fst)?;
        std::fs::write(path.join("index.meta.zst"), compressed_metadata)?;

        Ok(())
    }

    /// Load index using memory-mapped files (recommended, 4-5% faster)
    /// Memory-maps the compressed files, then decompresses into memory
    pub fn load_mmap(path: &Path) -> Result<Self> {
        let fst_file = File::open(path.join("index.fst.zst"))?;
        let meta_file = File::open(path.join("index.meta.zst"))?;

        // SAFETY: Files are read-only and won't be modified
        let fst_mmap = unsafe { Mmap::map(&fst_file)? };
        let meta_mmap = unsafe { Mmap::map(&meta_file)? };

        // Decompress from memory-mapped data
        let fst_bytes = zstd::decode_all(&fst_mmap[..])?;
        let metadata_bytes = zstd::decode_all(&meta_mmap[..])?;

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }

    /// Load index from disk with standard file read (fallback method)
    /// Slightly slower than load_mmap but simpler
    #[allow(dead_code)]
    pub fn load(path: &Path) -> Result<Self> {
        let compressed_fst = std::fs::read(path.join("index.fst.zst"))?;
        let compressed_metadata = std::fs::read(path.join("index.meta.zst"))?;

        let fst_bytes = zstd::decode_all(&compressed_fst[..])?;
        let metadata_bytes = zstd::decode_all(&compressed_metadata[..])?;

        Ok(IconIndex {
            fst_bytes,
            metadata_bytes,
        })
    }
}
