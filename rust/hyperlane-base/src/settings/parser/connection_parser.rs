use eyre::eyre;
use hyperlane_core::config::ConfigErrResultExt;
use hyperlane_core::{config::ConfigParsingError, HyperlaneDomainProtocol};
use url::Url;

use crate::settings::envs::*;
use crate::settings::ChainConnectionConf;

use super::{parse_base_and_override_urls, parse_cosmos_gas_price, ValueParser};

pub fn build_ethereum_connection_conf(
    rpcs: &[Url],
    chain: &ValueParser,
    err: &mut ConfigParsingError,
    default_rpc_consensus_type: &str,
) -> Option<ChainConnectionConf> {
    let Some(first_url) = rpcs.to_owned().clone().into_iter().next() else {
        return None;
    };
    let rpc_consensus_type = chain
        .chain(err)
        .get_opt_key("rpcConsensusType")
        .parse_string()
        .unwrap_or(default_rpc_consensus_type);

    match rpc_consensus_type {
        "single" => Some(h_eth::ConnectionConf::Http { url: first_url }),
        "fallback" => Some(h_eth::ConnectionConf::HttpFallback {
            urls: rpcs.to_owned().clone(),
        }),
        "quorum" => Some(h_eth::ConnectionConf::HttpQuorum {
            urls: rpcs.to_owned().clone(),
        }),
        ty => Err(eyre!("unknown rpc consensus type `{ty}`"))
            .take_err(err, || &chain.cwp + "rpc_consensus_type"),
    }
    .map(ChainConnectionConf::Ethereum)
}

pub fn build_cosmos_connection_conf(
    rpcs: &[Url],
    chain: &ValueParser,
    err: &mut ConfigParsingError,
) -> Option<ChainConnectionConf> {
    let mut local_err = ConfigParsingError::default();
    let grpcs =
        parse_base_and_override_urls(chain, "grpcUrls", "customGrpcUrls", "http", &mut local_err);

    let chain_id = chain
        .chain(&mut local_err)
        .get_key("chainId")
        .parse_string()
        .end()
        .or_else(|| {
            local_err.push(&chain.cwp + "chain_id", eyre!("Missing chain id for chain"));
            None
        });

    let prefix = chain
        .chain(err)
        .get_key("bech32Prefix")
        .parse_string()
        .end()
        .or_else(|| {
            local_err.push(
                &chain.cwp + "bech32Prefix",
                eyre!("Missing bech32 prefix for chain"),
            );
            None
        });

    let canonical_asset = if let Some(asset) = chain
        .chain(err)
        .get_opt_key("canonicalAsset")
        .parse_string()
        .end()
    {
        Some(asset.to_string())
    } else if let Some(hrp) = prefix {
        Some(format!("u{}", hrp))
    } else {
        local_err.push(
            &chain.cwp + "canonical_asset",
            eyre!("Missing canonical asset for chain"),
        );
        None
    };

    let gas_price = chain
        .chain(err)
        .get_opt_key("gasPrice")
        .and_then(parse_cosmos_gas_price)
        .end();

    let contract_address_bytes = chain
        .chain(err)
        .get_opt_key("contractAddressBytes")
        .parse_u64()
        .end();

    if !local_err.is_ok() {
        err.merge(local_err);
        None
    } else {
        Some(ChainConnectionConf::Cosmos(h_cosmos::ConnectionConf::new(
            grpcs,
            rpcs.first().unwrap().to_string(),
            chain_id.unwrap().to_string(),
            prefix.unwrap().to_string(),
            canonical_asset.unwrap(),
            gas_price.unwrap(),
            contract_address_bytes.unwrap().try_into().unwrap(),
        )))
    }
}

pub fn build_kadena_connection_conf(
    rpcs: &[Url],
    chain: &ValueParser,
    err: &mut ConfigParsingError,
) -> Option<ChainConnectionConf> {
    let mut local_err = ConfigParsingError::default();

    let account_name = chain
        .chain(&mut local_err)
        .get_opt_key("accountName")
        .parse_string()
        .end();

    let kadena_namespace = chain
        .chain(&mut local_err)
        .get_key("namespace")
        .parse_string()
        .end()
        .or_else(|| {
            local_err.push(&chain.cwp + "namespace", eyre!("Missing namespace"));
            None
        });

    let kadena_proxy_url = chain
        .chain(&mut local_err)
        .get_key("proxyUrl")
        .parse_from_str("Invalid kadena proxy url")
        .end()
        .or_else(|| {
            local_err.push(&chain.cwp + "proxy_url", eyre!("Missing proxyUrl"));
            None
        });

    rpcs.into_iter().next().and_then(|url| {
        let url_str = url.as_str();

        // Split URL at "chainweb" and get host part
        let host_with_scheme = url_str
            .rsplit("/chainweb/")
            .next()
            .map(|_| url_str[..url_str.rfind("/chainweb/").unwrap()].trim_end_matches('/'));

        if host_with_scheme.is_none() {
            local_err.push(
                &chain.cwp + "rpc_urls",
                eyre!("Kadena URL is not valid: chainweb segment not found"),
            );
            err.merge(local_err);
            return None;
        };

        // Get path segments for validation
        let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
        let chainweb_pos = path_segments.iter().rposition(|&s| s == "chainweb");

        if chainweb_pos.is_none() {
            local_err.push(
                &chain.cwp + "rpc_urls",
                eyre!("Kadena URL is not valid: chainweb segment not found"),
            );
            err.merge(local_err);
            return None;
        }
        let chainweb_pos = chainweb_pos.unwrap();

        // Validate remaining segments
        if path_segments.len() < chainweb_pos + 6 {
            local_err.push(
                &chain.cwp + "rpc_urls",
                eyre!("Invalid Kadena URL: insufficient segments after 'chainweb'"),
            );
            err.merge(local_err);
            return None;
        }

        if path_segments[chainweb_pos + 3] != "chain"
            || path_segments[chainweb_pos + 4].parse::<u8>().is_err()
        {
            local_err.push(
                &chain.cwp + "rpc_urls",
                eyre!("Invalid Kadena URL: invalid chain segments"),
            );
            err.merge(local_err);
            return None;
        }

        let network_id = path_segments[chainweb_pos + 2];
        let chain_id: u16 = path_segments[chainweb_pos + 4].parse().unwrap();

        if !local_err.is_ok() {
            err.merge(local_err);
            return None;
        }

        // Extract the host, network_id, and chain_id
        let host_with_scheme = host_with_scheme.unwrap();
        let kadena_proxy_url = kadena_proxy_url.unwrap();
        let kadena_namespace = kadena_namespace.unwrap().to_owned();
        let account_name = account_name.map(str::to_owned);

        Some(ChainConnectionConf::Kadena(h_kadena::ConnectionConf {
            url: url::Url::parse(host_with_scheme).unwrap(),
            network_id: network_id.to_string(),
            chain_id,
            kadena_proxy_url,
            kadena_namespace,
            account_name,
        }))
    })
}

pub fn build_connection_conf(
    domain_protocol: HyperlaneDomainProtocol,
    rpcs: &[Url],
    chain: &ValueParser,
    err: &mut ConfigParsingError,
    default_rpc_consensus_type: &str,
) -> Option<ChainConnectionConf> {
    match domain_protocol {
        HyperlaneDomainProtocol::Ethereum => {
            build_ethereum_connection_conf(rpcs, chain, err, default_rpc_consensus_type)
        }
        HyperlaneDomainProtocol::Fuel => rpcs
            .iter()
            .next()
            .map(|url| ChainConnectionConf::Fuel(h_fuel::ConnectionConf { url: url.clone() })),
        HyperlaneDomainProtocol::Sealevel => rpcs.iter().next().map(|url| {
            ChainConnectionConf::Sealevel(h_sealevel::ConnectionConf { url: url.clone() })
        }),
        HyperlaneDomainProtocol::Cosmos => build_cosmos_connection_conf(rpcs, chain, err),
        HyperlaneDomainProtocol::Kadena => build_kadena_connection_conf(rpcs, chain, err),
    }
}
