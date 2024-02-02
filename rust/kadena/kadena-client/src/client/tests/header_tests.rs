use super::*;

#[tokio::test]
async fn test_header_by_hash() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::HEADER_BY_HASH_PATH)
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "host".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.url.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "network".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.network_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "chain_id".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.chain_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded("hash".to_string(), "123".to_string()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&BlockHeaderDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let header_by_hash_result = client.header_by_hash("123".to_string()).await;

    assert!(
        header_by_hash_result.is_ok(),
        "header_by_hash request failed: {:?}",
        header_by_hash_result
    );

    let header_by_hash_result = header_by_hash_result.unwrap();
    assert_eq!(
        header_by_hash_result,
        BlockHeaderDto::default(),
        "header_by_hash result is not equal to 123"
    );
}

#[tokio::test]
async fn test_header_by_height() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::HEADER_BY_HEIGHT_PATH)
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "host".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.url.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "network".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.network_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "chain_id".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.chain_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded("height".to_string(), "123".to_string()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&BlockHeaderDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let header_by_height_result = client.header_by_height(123).await;

    assert!(
        header_by_height_result.is_ok(),
        "header_by_height request failed: {:?}",
        header_by_height_result
    );

    let header_by_height_result = header_by_height_result.unwrap();
    assert_eq!(
        header_by_height_result,
        BlockHeaderDto::default(),
        "header_by_height result is not equal to 123"
    );
}

#[tokio::test]
async fn test_headers() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::HEADERS_PATH)
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "host".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.url.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "network".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.network_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "chain_id".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.chain_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded("from".to_string(), "123".to_string()),
            mockito::Matcher::UrlEncoded("to".to_string(), "123".to_string()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&vec![BlockHeaderDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let headers_result = client.headers(123, 123).await;

    assert!(
        headers_result.is_ok(),
        "headers request failed: {:?}",
        headers_result
    );

    let headers_result = headers_result.unwrap();
    assert_eq!(
        headers_result,
        vec![BlockHeaderDto::default()],
        "headers result is not equal to 123"
    );
}
