use tokio::task::JoinHandle;

use super::entities::Response;
use super::http_repository::{HttpClientRepository, TaskRunningRequest};
use crate::app::services::http_collections::entities::requests::RequestData;
use crate::app::services::http_collections::entities::url::Url;

pub trait WebClient: Send + Sync {
    fn submit_request(&mut self, request: RequestData) -> JoinHandle<anyhow::Result<Response>>;
}

pub struct CoreWebClient {
    pub http_client: Box<dyn HttpClientRepository>,
}

impl CoreWebClient {
    pub fn init(repository: impl HttpClientRepository + 'static) -> Self {
        Self {
            http_client: Box::new(repository),
        }
    }
}

impl WebClient for CoreWebClient {
    fn submit_request(&mut self, mut request: RequestData) -> TaskRunningRequest {
        if let Url::ValidatedUrl(url) = &mut request.url {
            url.protocol.get_or_insert("http".to_string());
        }

        self.http_client.submit_request(request)
    }
}
