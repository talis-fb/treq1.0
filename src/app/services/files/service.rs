use std::{fs::OpenOptions, path::PathBuf};

use anyhow::Result;

pub trait FileService: Send + Sync {
    fn check_file_exists(&self, path: &PathBuf) -> Result<bool>;
    fn get_or_create_file(&self, path: &PathBuf) -> Result<PathBuf>;
    fn create_or_reset_file(&self, path: &PathBuf) -> Result<PathBuf>;
    fn remove_file(&self, path: &PathBuf) -> Result<()>;
    fn rename_file(&self, from: &PathBuf, to: &PathBuf) -> Result<()>;
    fn find_files_in_folder(&self, folder: &PathBuf) -> Result<Vec<PathBuf>>;
}

pub struct CoreFileService;

impl FileService for CoreFileService {
    fn remove_file(&self, path: &PathBuf) -> Result<()> {
        Ok(std::fs::remove_file(path)?)
    }

    fn rename_file(&self, from: &PathBuf, to: &PathBuf) -> Result<()> {
        Ok(std::fs::rename(from, to)?)
    }

    fn check_file_exists(&self, path: &PathBuf) -> Result<bool> {
        Ok(path.exists())
    }

    fn get_or_create_file(&self, path: &PathBuf) -> Result<PathBuf> {
        if !path.exists() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::File::create(&path)?;
        }
        Ok(path.clone())
    }

    fn create_or_reset_file(&self, path: &PathBuf) -> Result<PathBuf> {
        let file = OpenOptions::new()
            .write(true) 
            .create(true)
            .truncate(true)
            .open(&path)?;

        file.sync_all()?; 

        Ok(path.clone())
    }

    fn find_files_in_folder(&self, folder: &PathBuf) -> Result<Vec<PathBuf>> {
        Ok(std::fs::read_dir(folder)?
            .map(|entry| entry.unwrap().path())
            .collect())
    }
}
