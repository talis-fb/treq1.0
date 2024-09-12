use std::borrow::BorrowMut;
use std::future::Future;
use std::sync::Arc;

use anyhow::{Error, Result};
use async_trait::async_trait;
use tokio::sync::oneshot;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::sync::RwLockWriteGuard;

use crate::app::services::files::service::FileService;
use crate::app::services::files::service::CoreFileService;

use crate::app::services::http_client::service::WebClient;
use crate::app::services::http_client::service::CoreWebClient;
use crate::app::services::http_client::entities::Response;

use crate::app::services::http_collections::service::RequestService;
use crate::app::services::http_collections::service::CoreRequestService;
use crate::app::services::http_collections::entities::requests::RequestData;

use crate::utils::files as file_utils;
use crate::utils::uuid::UUID;

#[async_trait]
pub trait Backend: Send + Sync {
    async fn add_request(&mut self, request: RequestData) -> Result<UUID>;
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()>;
    async fn delete_request(&mut self, id: UUID) -> Result<()>;
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>>;
    async fn undo_request(&mut self, id: UUID) -> Result<()>;
    async fn redo_request(&mut self, id: UUID) -> Result<()>;
    async fn submit_request_async(
        &mut self,
        id: UUID,
    ) -> Result<oneshot::Receiver<Result<Response>>>;

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
    request_service: Arc<RwLock<Box<dyn RequestService>>>,
    web_client: Arc<RwLock<Box<dyn WebClient>>>,
    file_service: Arc<RwLock<Box<dyn FileService>>>,
}

impl AppBackend {
    pub fn init(
        request_service: impl RequestService + 'static,
        web_client: impl WebClient + 'static,
        file_service: impl FileService + 'static,
    ) -> Self {
        let request_service = Arc::new(RwLock::new(Box::new(request_service) as Box<dyn RequestService>));
        let web_client = Arc::new(RwLock::new(Box::new(web_client) as Box<dyn WebClient>));
        let file_service = Arc::new(RwLock::new(Box::new(file_service) as Box<dyn FileService>));
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
        let resp = self.request_service.write().await.add_request(request);
        Ok(resp)
    }
    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        self.request_service.write().await.edit_request(id, request);
        Ok(())
    }
    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        let request = self.request_service.write().await.get_request_data(id);
        Ok(request)
    }
    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        self.request_service.write().await.delete_request(id);
        Ok(())
    }
    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        self.request_service.write().await.undo_request_data(id);
        Ok(())
    }
    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        self.request_service.write().await.redo_request_data(id);
        Ok(())
    }

    async fn submit_request_async(&mut self, id: UUID) -> Result<Response> {
        // let request_data = self
        //     .get_request(id)
        //     .await?
        //     .ok_or(Error::msg("Not found request to given ID"))?;

        // let resp = run_command_waiting_response(
        //     &self.web_client,
        //     WebClientCommandsFactory::submit((*request_data).clone()),
        // )
        // .await?;
        // Ok(resp.unwrap())
        
        let request_data = self
            .get_request(id)
            .await?
            .ok_or(Error::msg("Not found request to given ID"))?;

        let resp = self.web_client.write().await.submit_request((*request_data).clone()).await?;
        resp
    }

    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()> {
        let path = self.file_service.write().await.get_or_create_data_file(name).await?;

        let request_data = serde_json::to_string(&request_data)?;
        file_utils::write_to_file(path, &request_data).await?;
        Ok(())
    }

    async fn get_request_saved(&mut self, name: String) -> Result<RequestData> {
        let path = run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::get_or_create_file_of_saved_request(name),
        )
        .await??;

        let request_data = file_utils::read_from_file(path.clone()).await?;
        if request_data.is_empty() {
            run_commands(
                &self.file_service,
                [FileServiceCommandsFactory::remove_file(path)],
            )
            .await?;
            return Err(Error::msg("This request does not exist"));
        }

        let request_data: RequestData = serde_json::from_str(&request_data)?;
        Ok(request_data)
    }

    async fn find_all_request_name(&mut self) -> Result<Vec<String>> {
        let response = run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::find_all_files_of_saved_requests(),
        )
        .await??;
        let file_names = response
            .into_iter()
            .map(|path| path.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        Ok(file_names)
    }

    async fn remove_request_saved(&mut self, name: String) -> Result<()> {
        run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::remove_file_saved_request(name),
        )
        .await?
    }

    async fn rename_request_saved(&mut self, request_name: String, new_name: String) -> Result<()> {
        run_command_waiting_response(
            &self.file_service,
            FileServiceCommandsFactory::rename_file_saved_request(request_name, new_name),
        )
        .await?
    }
}

// async fn run_commands<Service, Resp>(
//     service: &ServiceRunner<Service>,
//     commands: impl IntoIterator<Item = Command<Service, Resp>>,
// ) -> Result<()>
// where
//     Service: Send + 'static,
// {
//     for Command { command_fn, .. } in commands {
//         service.command_channel.send(command_fn).await?;
//     }
//     Ok(())
// }

// async fn run_command_waiting_response<Service, Resp>(
//     service: &ServiceRunner<Service>,
//     command: Command<Service, Resp>,
// ) -> Result<Resp>
// where
//     Service: Send + 'static,
// {
//     let Command {
//         command_fn,
//         response,
//     } = command;

//     service.command_channel.send(command_fn).await?;

//     match response {
//         Some(response_listener) => Ok(response_listener.await?),
//         None => Err(Error::msg("No response listener")),
//     }
// }
