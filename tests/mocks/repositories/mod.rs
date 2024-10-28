use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tempfile::{tempdir, TempDir};
use tokio::sync::oneshot;
use treq::app::kernel::{AppBackend, Backend};
use treq::app::services::files::service::{CoreFileService, FileService};
use treq::app::services::http_client::entities::Response;
use treq::app::services::http_client::http_repository::reqwest::ReqwestClientRepository;
use treq::app::services::http_client::service::{CoreWebClient, WebClient};
use treq::app::services::http_collections::entities::requests::RequestData;
use treq::app::services::http_collections::service::{CoreRequestService, RequestService};
use treq::utils::uuid::UUID;
use treq::view::output::writer::CliWriterRepository;
use treq::view::style::StyledStr;

pub fn create_mock_back_end() -> MockAppBackend {
    let temp_root = tempdir().unwrap();

    let config_dir = temp_root.path().join("config");
    let data_dir = temp_root.path().join("data");
    let tempfiles_dir = temp_root.path().join("tempfiles");

    [&config_dir, &data_dir, &tempfiles_dir]
        .iter()
        .filter(|dir| !dir.exists())
        .try_for_each(std::fs::create_dir_all)
        .unwrap();

    let req = CoreRequestService::init();
    let web = CoreWebClient::init(ReqwestClientRepository);
    let files = CoreFileService; //::init(config_dir, data_dir, tempfiles_dir);
    let backend = AppBackend::init(req, web, files);
    MockAppBackend::new(backend, temp_root)
}

pub struct MockAppBackend {
    app_backend: AppBackend,
    expected_requests: Vec<RequestData>,
    _temp_dir: TempDir,
}

impl MockAppBackend {
    pub fn new(app_backend: AppBackend, temp_dir: TempDir) -> Self {
        Self {
            app_backend,
            expected_requests: vec![],
            _temp_dir: temp_dir,
        }
    }

    pub fn with_expected_requests(
        mut self,
        expected_requests: impl IntoIterator<Item = RequestData>,
    ) -> Self {
        self.expected_requests = expected_requests.into_iter().collect();
        self
    }
}

#[async_trait]
impl Backend for MockAppBackend {
    async fn submit_http_request(&mut self, id: UUID) -> Result<Response> {
        let request = self.app_backend.get_request(id).await?.unwrap();
        let expected_request = self.expected_requests.remove(0);
        assert_eq!(Arc::new(expected_request), request);
        Ok(Response::default())
    }

    async fn add_request(&mut self, request: RequestData) -> Result<UUID> {
        self.app_backend.add_request(request).await
    }

    async fn edit_request(&mut self, id: UUID, request: RequestData) -> Result<()> {
        self.app_backend.edit_request(id, request).await
    }

    async fn get_request(&mut self, id: UUID) -> Result<Option<Arc<RequestData>>> {
        self.app_backend.get_request(id).await
    }

    async fn delete_request(&mut self, id: UUID) -> Result<()> {
        self.app_backend.delete_request(id).await
    }

    async fn undo_request(&mut self, id: UUID) -> Result<()> {
        self.app_backend.undo_request(id).await
    }

    async fn redo_request(&mut self, id: UUID) -> Result<()> {
        self.app_backend.redo_request(id).await
    }

    async fn save_request_datas_as(
        &mut self,
        name: String,
        request_data: RequestData,
    ) -> Result<()> {
        self.app_backend
            .save_request_datas_as(name, request_data)
            .await
    }

    async fn get_request_saved(&mut self, name: String) -> Result<RequestData> {
        self.app_backend.get_request_saved(name).await
    }

    async fn find_all_request_name(&mut self) -> Result<Vec<String>> {
        self.app_backend.find_all_request_name().await
    }

    async fn remove_request_saved(&mut self, name: String) -> Result<()> {
        self.app_backend.remove_request_saved(name).await
    }

    async fn rename_request_saved(&mut self, request_name: String, new_name: String) -> Result<()> {
        self.app_backend
            .rename_request_saved(request_name, new_name)
            .await
    }
}

pub struct CliWriterUseLess;

impl CliWriterRepository for CliWriterUseLess {
    fn clear_current_line(&mut self) {}

    fn print_lines<T: Display>(&mut self, _lines: impl IntoIterator<Item = T>) {}

    fn print_animation_single_line<T: Display, Sprites: IntoIterator<Item = T> + Sized + Clone>(
        &mut self,
        _sprites: Sprites,
        _interval: Duration,
        _finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone,
    {
    }

    fn print_centered_text_with_border(&mut self, _text: &str, _border_char: char) {}

    fn print_lines_styled<'a, StyledValues>(
        &mut self,
        _lines: impl IntoIterator<Item = StyledValues>,
    ) where
        StyledValues: IntoIterator<Item = StyledStr<'a>>,
    {
    }
}
