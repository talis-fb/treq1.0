use std::sync::Arc;

use crate::app::configurations::collections::USER_COLLECTIONS_FOLDER;
use crate::app::services::files::service::FileService;
use crate::app::services::service::Service;

pub struct RenameHttpCollection {
    pub file_service: Arc<Service<dyn FileService>>,
}

impl RenameHttpCollection {
    pub async fn execute(self, collection_name: String, new_name: String) -> anyhow::Result<()> {
        let from_path = USER_COLLECTIONS_FOLDER.clone().join(collection_name);
        let to_path = USER_COLLECTIONS_FOLDER.clone().join(new_name);

        let mut file_service = self.file_service.write().await;
        let file_service_ref = file_service.as_mut();

        file_service_ref.rename_file(&from_path, &to_path)?;
        Ok(())
    }
}
