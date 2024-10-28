use std::sync::Arc;


use crate::app::configurations::collections::USER_COLLECTIONS_FOLDER;
use crate::app::services::files::service::FileService;
use crate::app::services::http_collections::entities::requests::RequestData;
use crate::app::services::service::Service;
use crate::utils::files;

pub struct SaveHttpCollection {
    pub file_service: Arc<Service<dyn FileService>>,
}

impl SaveHttpCollection {
    pub async fn execute(
        self,
        collection_name: String,
        collection_data: RequestData,
    ) -> anyhow::Result<()> {
        let path = USER_COLLECTIONS_FOLDER.clone().join(collection_name);

        let mut file_service = self.file_service.write().await;
        let file_service_ref = file_service.as_mut();

        let file = file_service_ref.create_or_reset_file(&path)?;

        let collection_data_json = serde_json::to_string(&collection_data)?;

        files::write_to_file(file, &collection_data_json).await?;

        Ok(())
    }
}
