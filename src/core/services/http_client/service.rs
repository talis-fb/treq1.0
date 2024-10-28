use tokio::sync::oneshot::Receiver;

use super::entities::Response;
use super::http_repository::HttpClientRepository;
use crate::core::services::http_collections::entities::requests::RequestData;
use crate::core::services::http_collections::entities::url::Url;

pub trait WebClient: Send + Sync {
    fn submit_request(&mut self, request: RequestData) -> Receiver<anyhow::Result<Response>>;
}

pub struct CoreWebClient<R>
where
    R: HttpClientRepository,
{
    pub http_client: R,
}

impl<R> CoreWebClient<R>
where
    R: HttpClientRepository,
{
    pub fn init(repository: R) -> Self {
        Self {
            http_client: repository,
        }
    }
}

impl<R> WebClient for CoreWebClient<R>
where
    R: HttpClientRepository + Send + Sync,
{
    fn submit_request(&mut self, mut request: RequestData) -> Receiver<anyhow::Result<Response>> {
        if let Url::ValidatedUrl(url) = &mut request.url {
            url.protocol.get_or_insert("http".to_string());
        }

        self.http_client.submit_request(request)
    }
}
