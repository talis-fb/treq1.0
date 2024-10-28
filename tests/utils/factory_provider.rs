use treq::app::kernel::AppKernel;
use treq::app::services::files::service::CoreFileService;
use treq::app::services::http_client::http_repository::reqwest::ReqwestClientRepository;
use treq::app::services::http_client::http_repository::HttpClientRepository;
use treq::app::services::http_client::service::CoreWebClient;
use treq::app::services::http_collections::service::CoreRequestService;

pub async fn create_default_provider() -> AppKernel {
    let req = CoreRequestService::init();
    let web = CoreWebClient::init(ReqwestClientRepository);
    let files = CoreFileService; //::init(config_dir, data_dir, tempfiles_dir);
    AppKernel::init(req, web, files)
}

pub async fn create_provider_with_mock_web_client(
    web: impl HttpClientRepository + 'static + Sync,
) -> AppKernel {
    let req = CoreRequestService::init();
    let web = CoreWebClient::init(web);
    let files = CoreFileService; //::init(config_dir, data_dir, tempfiles_dir);
    AppKernel::init(req, web, files)
}
