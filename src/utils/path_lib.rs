use std::env;
use regex::Regex;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn expand_path(path: &str) -> PathBuf {
    let expanded_path = if path.starts_with("~") {
        if let Some(home_dir) = dirs_next::home_dir() {
            home_dir.join(&path[1..])
        } else {
            Path::new(path).to_path_buf()
        }
    } else {
        Path::new(path).to_path_buf()
    };

    let re = Regex::new(r"\$([A-Za-z0-9_]+)").unwrap();
    let expanded_str = expanded_path.to_string_lossy().into_owned();
    let fully_expanded = re.replace_all(&expanded_str, |caps: &regex::Captures| {
        env::var(&caps[1]).unwrap_or_default()
    });

    PathBuf::from(fully_expanded.as_ref())
}

pub fn walk<F>(root: &str, map_fn: Option<F>) -> Vec<PathBuf>
    where F: Fn(&DirEntry) -> Option<PathBuf>, {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|entry| map_fn.as_ref().and_then(|f| f(&entry)))
        .collect()
}
