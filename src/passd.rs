mod utils;

use rand;
use std::fs;
use utils::path_lib;
use walkdir::{DirEntry};
use utils::config::config;
use std::process::Command;
use std::path::{Path, PathBuf};

fn initialize(gpg_ids: Vec<&str>) {
    let gpg_ids = gpg_ids.join("\n");
    let password_store_dir = path_lib::expand_path(&config.store_dir);
    let gpg_id_path = format!("{}/.gpg-id", password_store_dir.display());

    match fs::create_dir(&password_store_dir) {
        Ok(_) => println!("Created password store in {}", password_store_dir.display()),
        Err(e) => eprintln!("Error creating directory {}: {}", password_store_dir.display() , e),
    }

    match fs::write(&gpg_id_path, &gpg_ids) {
        Ok(_) => println!("Initialized gpg-id"),
        Err(e) => eprintln!("Failed to initialize password store: {}", e),
    };

    encrypt_files(password_store_dir.to_str().unwrap());
}

fn encrypt_files(path: &str) {
	  let gpg_keys = "";
    let current_keys="";
    let prev_gpg_recipients = "";

    let gpg_files: Vec<PathBuf> = path_lib::walk(path, Some(|entry: &DirEntry| {
        let path = entry.path();

        if entry.file_type().is_symlink() { return None; }
        if path.to_string_lossy().contains(".git") { return None; }
        if path.to_string_lossy().contains(&config.extensions_dir) { return None; }
        if path.extension().map_or(false, |ext| ext == "gpg") { Some(path.to_path_buf()) }
        else { None }
    }));

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
}

fn verify_file(file_path: &str) {
    let signing_keys = &config.signing_keys;
    let sig_file = format!("{}.sig", file_path);

    if signing_keys.is_empty() { (); }
    if !fs::metadata(&sig_file).is_ok() { println!("Signature for {} does not exist.", sig_file); (); }
}

fn main() {
    initialize(vec!["nothong", "over", "here"])
}
