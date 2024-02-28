#[cfg(not(feature = "include-data"))]
fn main() {}

#[cfg(feature = "include-data")]
fn main() {
    data_repo::init_repo();
}

#[cfg(feature = "include-data")]
mod data_repo {
    use std::{env, fs, path::PathBuf};

    use cargo_toml::Manifest;
    use git2::{Oid, Repository};
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize)]
    struct Metadata {
        minecraft_data_repo: String,
        minecraft_data_commit: String,
    }

    pub fn init_repo() {
        println!("cargo:rerun-if-env-changed=MINECRAFT_DATA_REPO_PATH");
        println!("cargo:rerun-if-changed=Cargo.toml");

        let manifest = Manifest::<Metadata>::from_path_with_metadata(PathBuf::from("Cargo.toml"))
            .expect("Failed to read manifest (Cargo.toml)");
        let metadata = manifest
            .package
            .expect("missing package info in Cargo.toml")
            .metadata
            .expect("missing package.metadata in Cargo.toml");

        let repo_path = env::var("MINECRAFT_DATA_REPO_PATH")
            .map(PathBuf::from)
            .ok()
            .or_else(|| dirs::cache_dir().map(|p| p.join("minecraft-data")))
            .unwrap_or_else(|| PathBuf::from("minecraft-data"));

        println!(
            "cargo:rustc-env=MINECRAFT_DATA_PATH_INTERNAL={}",
            repo_path.to_string_lossy()
        );

        let version_oid = Oid::from_str(&metadata.minecraft_data_commit).expect("invalid oid");

        let repo = if repo_path.exists() {
            let repo = Repository::open(&repo_path).expect("failed to open git repo");
            let head_oid = repo
                .head()
                .expect("no head found in repo")
                .peel_to_commit()
                .expect("head is not a commit")
                .as_object()
                .id();
            if head_oid != version_oid {
                fs::remove_dir_all(&repo_path).expect("could not delete repository");
                Repository::clone(&metadata.minecraft_data_repo, repo_path)
                    .expect("failed to clone repo")
            } else {
                repo
            }
        } else {
            Repository::clone(&metadata.minecraft_data_repo, repo_path)
                .expect("failed to clone repo")
        };

        repo.set_head_detached(version_oid)
            .expect("failed set head");
        repo.checkout_head(None).expect("failed checkout index")
    }
}
