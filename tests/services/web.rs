use treq::app::kernel::Backend;
use treq::app::services::http_client::entities::Response;
use treq::app::services::http_client::http_repository::MockHttpClientRepository;
use treq::app::services::http_collections::entities::requests::RequestData;

use crate::utils::factory_provider::create_provider_with_mock_web_client;

#[tokio::test]
async fn test_basic_call_get() {
    fn expected_response() -> Response {
        let mut resp = Response::default();
        resp.status = 200;
        resp.body = "Ok".into();
        resp
    }

    fn expected_response_channel() -> tokio::sync::oneshot::Receiver<anyhow::Result<Response>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        tx.send(Ok(expected_response())).unwrap();
        rx
    }


    let mut mock_client = MockHttpClientRepository::new();
    mock_client
        .expect_submit_request()
        .times(1)
        .returning(move |_| expected_response_channel() );

    let mut provider = create_provider_with_mock_web_client(mock_client).await;
    let id_req = provider.add_request(RequestData::default()).await.unwrap();

    let response_submit = provider.submit_http_request(id_req).await.unwrap();

    assert_eq!(response_submit, expected_response());
}
