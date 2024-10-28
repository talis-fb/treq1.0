use std::borrow::{Borrow, BorrowMut};
use std::future::Future;
use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use tokio::sync::{oneshot, Mutex, RwLock, RwLockWriteGuard};

use crate::app::services::files::service::{CoreFileService, FileService};
use crate::app::services::http_client::entities::Response;
use crate::app::services::http_collections::entities::requests::RequestData;
use crate::app::services::http_collections::service::{CoreRequestService, RequestService};
use crate::utils::files as file_utils;
use crate::utils::uuid::UUID;

use super::commands::find_all_saved_http_collections_names::FindAllSavedHttpCollectionNames;
use super::commands::get_saved_http_collection::GetSavedHttpCollection;
use super::commands::remove_saved_http_collection::RemoveHttpCollection;
use super::commands::rename_saved_http_collection::RenameHttpCollection;
use super::commands::save_http_collection::SaveHttpCollection;
use super::services::http_client::service::WebClient;
use super::services::service::Service;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID>;
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()>;
    async fn delete_request(&mut self, id: UUID) -> Result<()>;
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>>;
    async fn undo_request(&mut self, id: UUID) -> Result<()>;
    async fn redo_request(&mut self, id: UUID) -> Result<()>;
    async fn submit_http_request(
        &mut self,
        id: UUID,
    ) -> Result<Response>;

    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()>;
    async fn get_request_saved(&mut self, name: String) -> Result<RequestData>;
    async fn find_all_request_name(&mut self) -> Result<Vec<String>>;
    async fn remove_request_saved(&mut self, name: String) -> Result<()>;
    async fn rename_request_saved(&mut self, request_name: String, new_name: String) -> Result<()>;
}

pub struct AppBackend {
    request_service: Arc<Service<dyn RequestService>>,
    web_client: Arc<Service<dyn WebClient>>,
    file_service: Arc<Service<dyn FileService>>,
}

impl AppBackend {
    pub fn init(
        request_service: impl RequestService + 'static,
        web_client: impl WebClient + 'static,
        file_service: impl FileService + 'static,
    ) -> Self {
        let request_service = Arc::new(Service::from(request_service));
        let web_client = Arc::new(Service::from(web_client));
        let file_service = Arc::new(Service::from(file_service));
        Self {
            request_service,
            web_client,
            file_service,
        }
    }
}

#[async_trait]
impl Backend for AppBackend {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID> {
        let resp = self.request_service.write().await.as_mut().add_request(request);
        Ok(resp)
    }
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        self.request_service.write().await.as_mut().edit_request(id, request);
        Ok(())
    }
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        let request = self.request_service.write().await.as_mut().get_request_data(id);
        Ok(request)
    }
    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        self.request_service.write().await.as_mut().delete_request(id);
        Ok(())
    }
    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        self.request_service.write().await.as_mut().undo_request_data(id);
        Ok(())
    }
    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        self.request_service.write().await.as_mut().redo_request_data(id);
        Ok(())
    }

    async fn submit_http_request(&mut self, id: UUID) -> Result<Response> {
        let request_data = self
            .get_request(id)
            .await?
            .ok_or(Error::msg("Not found request to given ID"))?;

        let resp = self
            .web_client
            .write()
            .await
            .as_mut()
            .submit_request((*request_data).clone())
            .await??;

        Ok(resp)
    }

    async fn save_request_datas_as(
        &mut self,
        collection_name: String,
        collection_data: RequestData,
    ) -> Result<()> {
        SaveHttpCollection {
            file_service: self.file_service.clone(),
        }.execute(collection_name, collection_data).await
    }

    async fn get_request_saved(&mut self, collection_name: String) -> Result<RequestData> {
        GetSavedHttpCollection {
            file_service: self.file_service.clone(),
        }.execute(collection_name).await
    }

    async fn find_all_request_name(&mut self) -> Result<Vec<String>> {
        FindAllSavedHttpCollectionNames {
            file_service: self.file_service.clone(),
        }.execute().await
    }

    async fn remove_request_saved(&mut self, name: String) -> Result<()> {
        RemoveHttpCollection {
            file_service: self.file_service.clone(),
        }.execute(name).await
    }

    async fn rename_request_saved(&mut self, collection_name: String, new_name: String) -> Result<()> {
        RenameHttpCollection {
            file_service: self.file_service.clone(),
        }.execute(collection_name, new_name).await
    }
}
