use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn walk_dir_and_match_ext(path: &Path, ext: &str) -> Vec<DirEntry> {
    WalkDir::new(path)
        .min_depth(2)
        .into_iter()
        .filter_map(|i| {
            i.ok()
                .filter(|j| j.path().extension().filter(|k| *k == ext).is_some())
        })
        .collect()
}
