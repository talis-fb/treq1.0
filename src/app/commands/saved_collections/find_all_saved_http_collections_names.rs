use std::sync::Arc;

use anyhow::Error;

use crate::app::configurations::collections::HTTP_COLLECTIONS_FOLDER;
use crate::app::services::files::service::FileService;
use crate::app::services::service::Service;

pub struct FindAllSavedHttpCollectionNames {
    pub file_service: Arc<Service<dyn FileService>>,
}

impl FindAllSavedHttpCollectionNames {
    pub async fn execute(self) -> anyhow::Result<Vec<String>> {
        let path = HTTP_COLLECTIONS_FOLDER;

        let mut file_service = self.file_service.write().await;
        let file_service_ref = file_service.as_mut();

        let files = file_service_ref.find_files_in_folder(path.into())?;
        Ok(files
            .iter()
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
            .collect())
    }
}
