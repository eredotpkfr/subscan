use wiremock::{MockBuilder, MockServer, ResponseTemplate};

pub async fn test_server_with_response(
    mock: MockBuilder,
    response: ResponseTemplate,
) -> MockServer {
    let server = MockServer::start().await;

    mock.respond_with(response).mount(&server).await;
    server
}
