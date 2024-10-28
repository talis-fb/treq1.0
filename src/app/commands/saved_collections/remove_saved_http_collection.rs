use std::sync::Arc;

use anyhow::Error;

use crate::app::configurations::collections::{HTTP_COLLECTIONS_FOLDER, USER_COLLECTIONS_FOLDER};
use crate::app::services::files::service::FileService;
use crate::app::services::http_collections::entities::requests::RequestData;
use crate::app::services::service::Service;
use crate::utils::files;

pub struct RemoveHttpCollection {
    pub file_service: Arc<Service<dyn FileService>>,
}

impl RemoveHttpCollection {
    pub async fn execute(self, collection_name: String) -> anyhow::Result<()> {
        let path = USER_COLLECTIONS_FOLDER.clone().join(collection_name);

        let mut file_service = self.file_service.write().await;
        let file_service_ref = file_service.as_mut();

        file_service_ref.remove_file(&path)?;
        Ok(())
    }
}
