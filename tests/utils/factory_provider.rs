use treq::app::kernel::AppBackend;
use treq::app::services::files::service::FileService;
use treq::app::services::http_client::http_repository::reqwest::ReqwestClientRepository;
use treq::app::services::http_client::http_repository::HttpClientRepository;
use treq::app::services::http_client::service::WebClient;
use treq::app::services::http_collections::service::RequestService;

pub async fn create_default_provider() -> AppBackend {
    let req = RequestService::init();
    let web = WebClient::init(ReqwestClientRepository);
    let files = FileService::init("", "", "");
    AppBackend::init(req, web, files)
}

pub async fn create_provider_with_mock_web_client(
    web: impl HttpClientRepository + 'static,
) -> AppBackend {
    let req = RequestService::init();
    let web = WebClient::init(web);
    let files = FileService::init("", "", "");
    AppBackend::init(req, web, files)
}
