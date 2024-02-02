use super::*;

#[tokio::test]
async fn test_build_tx_success() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::BUILD_TX_PATH)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&CommandDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let build_tx_result = client.build_tx(BuildPactTxDto::default()).await;
    assert!(
        build_tx_result.is_ok(),
        "build_tx failed: {:?}",
        build_tx_result
    );

    let build_tx_result = build_tx_result.unwrap();
    assert_eq!(
        build_tx_result,
        CommandDto::default(),
        "build_tx result is not equal to mocked_command_dto"
    );
}

#[tokio::test]
async fn test_build_tx_error() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::BUILD_TX_PATH)
        .with_status(500)
        .with_header("content-type", "application/json")
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let build_tx_result = client.build_tx(BuildPactTxDto::default()).await;
    assert!(
        build_tx_result.is_err(),
        "build_tx passed, but it should fail: {:?}",
        build_tx_result
    );
}

#[tokio::test]
async fn test_poll_success() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::POLL_PATH)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            serde_json::to_string(
                &[("dummy key".to_string(), CommandResultDto::default())]
                    .into_iter()
                    .collect::<HashMap<String, CommandResultDto>>(),
            )
            .unwrap(),
        )
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let poll_result = client.poll(PollRequestBodyDto::default()).await;
    assert!(
        poll_result.is_ok(),
        "poll request failed: {:?}",
        poll_result
    );

    let poll_result = poll_result.unwrap();
    assert_eq!(
        poll_result,
        [("dummy key".to_string(), CommandResultDto::default())]
            .into_iter()
            .collect::<HashMap<String, CommandResultDto>>(),
        "poll result is not equal to mocked_command_dto"
    );
}

#[tokio::test]
async fn test_poll_error() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::POLL_PATH)
        .with_status(500)
        .with_header("content-type", "application/json")
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let poll_result = client.poll(PollRequestBodyDto::default()).await;
    assert!(
        poll_result.is_err(),
        "poll request passed, but it should fail: {:?}",
        poll_result
    );
}

#[tokio::test]
async fn test_local_with_continuation_success() {
    let mut server = mockito::Server::new_async().await;

    let local_response_with_continuation = r#"
        {
            "gas": 646,
            "result": {
              "status": "success",
              "data": true
            },
            "reqKey": "lDrEdMoGHKco8hwa0Mtn3FJzQFA48PgR8p-7VXuoSx8",
            "logs": "5twyJC8GqByfC8d7yNWdlCoDXlGnzrPmdgPT3xRpuEU",
            "events": [
              {
                "params": [
                  "sender00",
                  "NoMiner",
                  0.00000646
                ],
                "name": "TRANSFER",
                "module": {
                  "namespace": null,
                  "name": "coin"
                },
                "moduleHash": "M1gabakqkEi_1N8dRKt4z5lEv1kuC_nxLTnyDCuZIK0"
              },
              {
                "params": [
                  "1",
                  "free.hyp-erc20.transfer-to-crosschain",
                  [
                    "k:94c35ab1bd70243ec670495077f7846373b4dc5e9779d7a6732b5ceb6fde059c",
                    10,
                    "1"
                  ]
                ],
                "name": "X_YIELD",
                "module": {
                  "namespace": null,
                  "name": "pact"
                },
                "moduleHash": "uTJNAjkFEbH9iO-Fhfqri0r-IIy0PSMtN6gMpiqJFyY"
              },
              {
                "params": [
                  "31337",
                  "k:94c35ab1bd70243ec670495077f7846373b4dc5e9779d7a6732b5ceb6fde059c",
                  10
                ],
                "name": "RECEIVED_TRANSFER_REMOTE",
                "module": {
                  "namespace": "free",
                  "name": "hyp-erc20"
                },
                "moduleHash": "uTJNAjkFEbH9iO-Fhfqri0r-IIy0PSMtN6gMpiqJFyY"
              },
              {
                "params": [
                  "31337",
                  "0x740b133dedb75bdb58d000054e873cae6fc565fb",
                  "6YKzqpDNATmPhUJzc5A17mJbFXH-dBkV"
                ],
                "name": "PROCESS",
                "module": {
                  "namespace": "free",
                  "name": "mailbox"
                },
                "moduleHash": "CHKV64Wrvg1QEvjNgU3j8LXWo3kwzN9JA3EF3diwWbs"
              },
              {
                "params": [
                  "0xdb764dbdfbc14f5ff2e03db8382e164a3a13c1563fe43461a0ad48b8c1d5a6c5"
                ],
                "name": "PROCESS-ID",
                "module": {
                  "namespace": "free",
                  "name": "mailbox"
                },
                "moduleHash": "CHKV64Wrvg1QEvjNgU3j8LXWo3kwzN9JA3EF3diwWbs"
              }
            ],
            "metaData": {
              "publicMeta": {
                "creationTime": 1706497850,
                "ttl": 30000,
                "gasLimit": 100000,
                "chainId": "0",
                "gasPrice": 1e-8,
                "sender": "sender00"
              },
              "blockTime": 1706526648844156,
              "prevBlockHash": "wbF6zWOvBKCW4wLoYwSjv_v6Uzb09bQhyvK8YLNSc48",
              "blockHeight": 1236
            },
            "continuation": {
              "executed": null,
              "pactId": "lDrEdMoGHKco8hwa0Mtn3FJzQFA48PgR8p-7VXuoSx8",
              "stepHasRollback": false,
              "step": 0,
              "yield": {
                "data": {
                  "amount": 10,
                  "receiver": "k:94c35ab1bd70243ec670495077f7846373b4dc5e9779d7a6732b5ceb6fde059c"
                },
                "source": "0",
                "provenance": {
                  "targetChainId": "1",
                  "moduleHash": "uTJNAjkFEbH9iO-Fhfqri0r-IIy0PSMtN6gMpiqJFyY"
                }
              },
              "continuation": {
                "args": [
                  "k:94c35ab1bd70243ec670495077f7846373b4dc5e9779d7a6732b5ceb6fde059c",
                  10,
                  "1"
                ],
                "def": "free.hyp-erc20.transfer-to-crosschain"
              },
              "stepCount": 2
            },
            "txId": 1370
          }"#;

    server
        .mock("POST", Endpoint::LOCAL_PATH)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(local_response_with_continuation)
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let local_result = client.local(LocalRequestBodyDto::default()).await;
    assert!(
        local_result.is_ok(),
        "local request failed: {:?}",
        local_result
    );

    let local_result = local_result.unwrap();

    let local_result_expected = serde_json::from_str(local_response_with_continuation)
        .expect("failed to deserialize local_response_with_continuation");

    assert_eq!(
        local_result, local_result_expected,
        "local result is not equal to mocked_command_dto"
    );
}

#[tokio::test]
async fn test_local_success() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::LOCAL_PATH)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&CommandResultDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let local_result = client.local(LocalRequestBodyDto::default()).await;
    assert!(
        local_result.is_ok(),
        "local request failed: {:?}",
        local_result
    );

    let local_result = local_result.unwrap();
    assert_eq!(
        local_result,
        CommandResultDto::default(),
        "local result is not equal to mocked_command_dto"
    );
}

#[tokio::test]
async fn test_local_error() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::LOCAL_PATH)
        .with_status(500)
        .with_header("content-type", "application/json")
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let local_result = client.local(LocalRequestBodyDto::default()).await;
    assert!(
        local_result.is_err(),
        "local request passed, but it should fail: {:?}",
        local_result
    );
}

#[tokio::test]
async fn test_send_success() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::SEND_PATH)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&RequestKeysDto::default()).unwrap())
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );
    let send_result = client.send(SendRequestBodyDto::default()).await;
    assert!(
        send_result.is_ok(),
        "send request failed: {:?}",
        send_result
    );

    let send_result = send_result.unwrap();
    assert_eq!(
        send_result,
        RequestKeysDto::default(),
        "send result is not equal to mocked_command_dto"
    );
}

#[tokio::test]
async fn test_send_error() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("POST", Endpoint::SEND_PATH)
        .with_status(500)
        .with_header("content-type", "application/json")
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );
    let send_result = client.send(SendRequestBodyDto::default()).await;
    assert!(
        send_result.is_err(),
        "send request passed, but it should fail: {:?}",
        send_result
    );
}

#[tokio::test]
async fn test_height_success() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::HEIGHT_PATH)
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "host".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.url.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "network".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.network_id.to_string(),
            ),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("123")
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let height_result = client.height(None).await;

    assert!(
        height_result.is_ok(),
        "height request failed: {:?}",
        height_result
    );

    let height_result = height_result.unwrap();
    assert_eq!(height_result, 123, "height result is not equal to 123");
}

#[tokio::test]
async fn test_height_error() {
    let mut server = mockito::Server::new_async().await;

    server
        .mock("GET", Endpoint::HEIGHT_PATH)
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded(
                "host".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.url.to_string(),
            ),
            mockito::Matcher::UrlEncoded(
                "network".to_string(),
                DEFAULT_CONTEXT.chainweb_conf.network_id.to_string(),
            ),
            mockito::Matcher::UrlEncoded("depth".to_string(), "0".to_string()), // there shoudn't be depth, so we will get an error
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("123")
        .create();

    let client = KadenaProxyClient::new(
        ProxyConf::new_with_url(Url::parse(&server.url()).unwrap()),
        DEFAULT_CONTEXT.chainweb_conf.clone(),
    );

    let height_result = client.height(None).await;

    assert!(
        height_result.is_err(),
        "height request passed, but it should fail: {:?}",
        height_result
    );
}
