use std::{path::PathBuf, sync::LazyLock};
use anyhow::Error;
use directories::ProjectDirs;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

pub const COLLECTIONS_FOLDER: &str = "v1/collection/";
pub const HTTP_COLLECTIONS_FOLDER: &str = "v1/collection/http/";

pub static USER_COLLECTIONS_FOLDER: LazyLock<PathBuf> = LazyLock::new(|| {
    let proj_dirs = ProjectDirs::from("com", APP_AUTHOR, APP_NAME).ok_or(Error::msg(
        "No possible to create or access directories of data and configuration",
    )).unwrap();

    let data_dir = proj_dirs.data_dir().to_path_buf();

    let collections_dir = data_dir.join(COLLECTIONS_FOLDER);
    
    collections_dir
});