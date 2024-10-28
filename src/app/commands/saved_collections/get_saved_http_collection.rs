use std::sync::Arc;

use anyhow::Error;

use crate::app::configurations::collections::HTTP_COLLECTIONS_FOLDER;
use crate::app::services::files::service::FileService;
use crate::app::services::http_collections::entities::requests::RequestData;
use crate::app::services::service::Service;
use crate::utils::files;

pub struct GetSavedHttpCollection {
    pub file_service: Arc<Service<dyn FileService>>,
}

impl GetSavedHttpCollection {
    pub async fn execute(self, collection_name: String) -> anyhow::Result<RequestData> {
        let path = format!("{HTTP_COLLECTIONS_FOLDER}/{collection_name}");

        let mut file_service = self.file_service.write().await;
        let file_service_ref = file_service.as_mut();

        let file_buf = file_service_ref.get_or_create_file(path.into())?;
        let file_content = files::read_from_file(&file_buf).await?;

        if file_content.is_empty() {
            file_service_ref.remove_file(file_buf)?;
            return Err(Error::msg("This request does not exist"));
        }

        let request_data: RequestData = serde_json::from_str(&file_content)?;
        Ok(request_data)
    }
}
