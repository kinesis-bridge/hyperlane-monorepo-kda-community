mod block_tests;
mod event_tests;
mod general_tests;
mod header_tests;
mod tx_tests;

use super::*;
use once_cell::sync::Lazy;

struct DefaultContext {
    chainweb_conf: ChainwebConf,
}

static DEFAULT_CONTEXT: Lazy<DefaultContext> = Lazy::new(|| {
    let chainweb_conf = ChainwebConf {
        url: Url::parse("http://kadena:8080").unwrap(),
        network_id: "development".to_owned(),
        chain_id: 0,
    };
    DefaultContext { chainweb_conf }
});
