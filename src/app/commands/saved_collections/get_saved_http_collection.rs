use anyhow::Error;

use crate::app::commands::{AsyncCommand, Command};
use crate::app::configurations::collections::HTTP_COLLECTIONS_FOLDER;
use crate::app::services::files::service::FileService;
use crate::app::services::http_collections::entities::requests::RequestData;
use crate::utils::files;

pub struct GetSavedHttpCollection<'a> {
    pub file_service: &'a mut dyn FileService,
}

impl<'a> AsyncCommand for GetSavedHttpCollection<'a> {
    type Input = &'a str;
    type Output = anyhow::Result<RequestData>;

    async fn execute(self, collection_name: Self::Input) -> Self::Output {
        let path = format!("{HTTP_COLLECTIONS_FOLDER}/{collection_name}");
        let file_buf = self.file_service.get_or_create_data_file(path)?;
        let file_content = files::read_from_file(&file_buf).await?;

        if file_content.is_empty() {
            self.file_service.remove_file(file_buf)?;
            return Err(Error::msg("This request does not exist"));
        }

        let request_data: RequestData = serde_json::from_str(&file_content)?;
        Ok(request_data)
    }
}
