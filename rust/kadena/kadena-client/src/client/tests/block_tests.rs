use super::*;

#[tokio::test]
async fn test_block_by_hash() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::BLOCK_BY_HASH_PATH)
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
        .with_body(serde_json::to_string(&BlockPayloadsDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
        DEFAULT_CONTEXT.contracts_conf.clone(),
    );
    let block_by_hash_result = client.block_by_hash("123".to_string()).await;

    assert!(
        block_by_hash_result.is_ok(),
        "block_by_hash request failed: {:?}",
        block_by_hash_result
    );

    let block_by_hash_result = block_by_hash_result.unwrap();
    assert_eq!(
        block_by_hash_result,
        BlockPayloadsDto::default(),
        "block_by_hash result is not equal to 123"
    );
}

#[tokio::test]
async fn test_block_by_height() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::BLOCK_BY_HEIGHT_PATH)
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
        .with_body(serde_json::to_string(&BlockPayloadsDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
        DEFAULT_CONTEXT.contracts_conf.clone(),
    );
    let block_by_height_result = client.block_by_height(123).await;

    assert!(
        block_by_height_result.is_ok(),
        "block_by_height request failed: {:?}",
        block_by_height_result
    );

    let block_by_height_result = block_by_height_result.unwrap();
    assert_eq!(
        block_by_height_result,
        BlockPayloadsDto::default(),
        "block_by_height result is not equal to 123"
    );
}

#[tokio::test]
async fn test_blocks() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::BLOCKS_PATH)
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
        .with_body(serde_json::to_string(&vec![BlockPayloadsDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
        DEFAULT_CONTEXT.contracts_conf.clone(),
    );
    let blocks_result = client.blocks(123, 123).await;

    assert!(
        blocks_result.is_ok(),
        "blocks request failed: {:?}",
        blocks_result
    );

    let blocks_result = blocks_result.unwrap();
    assert_eq!(
        blocks_result,
        vec![BlockPayloadsDto::default()],
        "blocks result is not equal to 123"
    );
}
