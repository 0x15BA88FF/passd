use passd::commands::initialize;
use std::{fs, io};
use tempfile::tempdir;

#[test]
fn test_initialize() -> Result<(), io::Error> {
    let temp_dir = tempdir()?;
    let store_path = temp_dir.path().join("store");
    let pgp_keys = vec![
        "84DFC49A7A542FE385938DCAA6BD563A5FB3E2C7",
        "82D910EE90DD22508192D5BB27E7044A3BEEB8ED",
    ];

    initialize(&store_path, pgp_keys.clone())?;

    assert!(store_path.exists(), "The store directory should exist.");

    let gpg_id_path = store_path.join(".gpg-id");

    assert!(
        gpg_id_path.exists(),
        "The .gpg-id file should exist in the store directory."
    );

    let gpg_id_content = fs::read_to_string(gpg_id_path)?;

    assert_eq!(
        gpg_id_content,
        pgp_keys.join("\n"),
        "The .gpg-id file content should match the provided PGP keys."
    );

    Ok(())
}
