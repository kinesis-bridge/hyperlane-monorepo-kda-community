use super::*;

#[tokio::test]
async fn test_event_by_hash() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::EVENT_BY_HASH_PATH)
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
        .with_body(serde_json::to_string(&vec![EventDataDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
        DEFAULT_CONTEXT.contracts_conf.clone(),
    );
    let event_by_hash_result = client.event_by_hash("123".to_string()).await;

    assert!(
        event_by_hash_result.is_ok(),
        "event_by_hash request failed: {:?}",
        event_by_hash_result
    );

    let event_by_hash_result = event_by_hash_result.unwrap();
    assert_eq!(
        event_by_hash_result,
        vec![EventDataDto::default()],
        "event_by_hash result is not equal to 123"
    );
}

#[tokio::test]
async fn test_event_by_height() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::EVENT_BY_HEIGHT_PATH)
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
        .with_body(serde_json::to_string(&vec![EventDataDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
        DEFAULT_CONTEXT.contracts_conf.clone(),
    );
    let event_by_height_result = client.event_by_height(123).await;

    assert!(
        event_by_height_result.is_ok(),
        "event_by_height request failed: {:?}",
        event_by_height_result
    );

    let event_by_height_result = event_by_height_result.unwrap();
    assert_eq!(
        event_by_height_result,
        vec![EventDataDto::default()],
        "event_by_height result is not equal to 123"
    );
}

#[tokio::test]
async fn test_events() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::EVENTS_PATH)
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
        .with_body(serde_json::to_string(&vec![EventDataDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
        DEFAULT_CONTEXT.contracts_conf.clone(),
    );
    let events_result = client.events(123, 123).await;

    assert!(
        events_result.is_ok(),
        "events request failed: {:?}",
        events_result
    );

    let events_result = events_result.unwrap();
    assert_eq!(
        events_result,
        vec![EventDataDto::default()],
        "events result is not equal to 123"
    );
}
