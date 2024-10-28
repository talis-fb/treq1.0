pub mod reqwest;

use mockall::automock;
use mockall::predicate::*;
use tokio::sync::oneshot::Receiver;

use super::entities::Response;
use crate::app::services::http_collections::entities::requests::RequestData;

// -------------------------------------------------------------------------------------------------------------------
// TODO: Make this 'automock' enabled only in test mode
//  and also move the dependency definition to dev-dependencies,
//  but doing it now breaks web_client's integration tests. At importing of Mock HttpRepository
// -------------------------------------------------------------------------------------------------------------------

#[automock]
pub trait HttpClientRepository: Send {
    fn submit_request(&self, request: RequestData) -> Receiver<anyhow::Result<Response>>;
}
