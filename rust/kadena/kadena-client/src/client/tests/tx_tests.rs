use super::*;

#[tokio::test]
async fn test_tx_by_hash() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::TX_BY_HASH_PATH)
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
        .with_body(serde_json::to_string(&vec![TransactionElementDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );
    let tx_by_hash_result = client.tx_by_hash("123".to_string()).await;

    assert!(
        tx_by_hash_result.is_ok(),
        "tx_by_hash request failed: {:?}",
        tx_by_hash_result
    );

    let tx_by_hash_result = tx_by_hash_result.unwrap();
    assert_eq!(
        tx_by_hash_result,
        vec![TransactionElementDto::default()],
        "tx_by_hash result is not equal to 123"
    );
}

#[tokio::test]
async fn test_tx_by_height() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::TX_BY_HEIGHT_PATH)
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
        .with_body(serde_json::to_string(&vec![TransactionElementDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );
    let tx_by_height_result = client.tx_by_height(123).await;

    assert!(
        tx_by_height_result.is_ok(),
        "tx_by_height request failed: {:?}",
        tx_by_height_result
    );

    let tx_by_height_result = tx_by_height_result.unwrap();
    assert_eq!(
        tx_by_height_result,
        vec![TransactionElementDto::default()],
        "tx_by_height result is not equal to 123"
    );
}

#[tokio::test]
async fn test_txs() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::TXS_PATH)
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
        .with_body(serde_json::to_string(&vec![TransactionElementDto::default()]).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );
    let txs_result = client.txs(123, 123).await;

    assert!(txs_result.is_ok(), "txs request failed: {:?}", txs_result);

    let txs_result = txs_result.unwrap();
    assert_eq!(
        txs_result,
        vec![TransactionElementDto::default()],
        "txs result is not equal to 123"
    );
}
