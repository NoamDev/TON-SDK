#![recursion_limit="128"] // needed for error_chain

#[macro_use]
extern crate tvm;
extern crate ton_abi;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate hex;
extern crate ed25519_dalek;
extern crate rand;
extern crate sha2;
extern crate base64;
extern crate crc16;
extern crate chrono;

#[cfg(feature = "node_interaction")]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
#[cfg(feature = "node_interaction")]
#[macro_use]
extern crate serde_json;
#[cfg(feature = "node_interaction")]
extern crate futures;
#[cfg(feature = "node_interaction")]
extern crate graphite;

pub use ton_abi::json_abi;
pub use ton_abi::Contract as AbiContract;
pub use ton_abi::Function as AbiFunction;

#[allow(deprecated)]
#[macro_use]
mod error;
pub use error::*;

mod contract;
pub use contract::*;

mod message;
pub use message::*;

mod local_tvm;

#[cfg(feature = "node_interaction")]
mod transaction;
#[cfg(feature = "node_interaction")]
pub use transaction::*;
/*
#[cfg(feature = "node_interaction")]
mod block;
#[cfg(feature = "node_interaction")]
pub use block::*;
*/
mod types;
pub use types::*;

#[cfg(feature = "node_interaction")]
pub mod queries_helper;
#[cfg(feature = "node_interaction")]
mod requests_helper;

pub mod json_helper;


/// Init SKD. Globally saves queries and requests server URLs
#[cfg(feature = "node_interaction")]
pub fn init(config: NodeClientConfig) -> SdkResult<()> {
    requests_helper::init(config.requests_config);
    queries_helper::init(config.queries_config);
    Ok(())
}

/// Init SKD. Globally saves queries and requests server URLs
#[cfg(feature = "node_interaction")]
pub fn init_json(config: &str) -> SdkResult<()> {
    init(serde_json::from_str(config)
        .map_err(|err| SdkErrorKind::InvalidArg(format!("{}", err)))?)
}

/// Uninit SKD. Should be called before process
#[cfg(feature = "node_interaction")]
pub fn uninit() {
    requests_helper::uninit();
    queries_helper::uninit();
}

#[cfg(test)]
#[path = "tests/test_lib.rs"]
mod tests;

#[cfg(test)]
#[path = "tests/test_piggy_bank.rs"]
mod test_piggy_bank;

#[cfg(test)]
#[path = "tests/tests_common.rs"]
mod tests_common;
