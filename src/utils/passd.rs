use crate::utils::config::CONFIG;
use crate::utils::path_lib;
use rand;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};
use walkdir::DirEntry;

pub fn encrypt_files(path: &str) -> Result<(), io::Error> {
    let current_keys = "";
    let mut gpg_keys = "";
    let mut prev_gpg_recipients = "";

    let gpg_files: Vec<PathBuf> = path_lib::walk(
        path,
        Some(|entry: &DirEntry| {
            let path = entry.path();
            if entry.file_type().is_symlink() {
                return None;
            }
            if path.to_string_lossy().contains(".git") {
                return None;
            }
            if path
                .to_string_lossy()
                .contains(&CONFIG.extensions_directory)
            {
                return None;
            }
            if path.extension().map_or(false, |ext| ext == "gpg") {
                Some(path.to_path_buf())
            } else {
                None
            }
        }),
    );

    for file in gpg_files {
        let file_dir = file.parent().unwrap_or(Path::new(""));
        println!("Processing file in directory: {}", file_dir.display());

        let temp_file_path = format!(
            "{}.tmp.{}.{}.{}.{}",
            file.display(),
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>(),
            rand::random::<u16>()
        );

        verify_file(&format!("{}/.gpg-id", path))
    }

    Ok(())
}

pub fn verify_file(file_path: &str) {
    let signing_keys = &CONFIG.signing_keys;
    let sig_file = format!("{}.sig", file_path);

    if signing_keys.is_empty() {
        ();
    }
    if !fs::metadata(&sig_file).is_ok() {
        println!("Signature for {} does not exist.", sig_file);
        ();
    }
}
