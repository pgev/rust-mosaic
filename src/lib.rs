// Copyright 2018 OpenST Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This crate implements a mosaic node.
//! Mosaic nodes run to:
//!  - validate utility systems
//!  - commit a value chain onto a utility chain
//!  - commit a utility chain onto a value chain

#[macro_use]
extern crate log;
extern crate web3;

use std::env;
use std::error::Error;

mod blockchain;

// Environment variables and their defaults
const ENV_ORIGIN_ADDRESS: &str = "MOSAIC_ORIGIN_ADDRESS";
const ENV_AUXILIARY_ADDRESS: &str = "MOSAIC_AUXILIARY_ADDRESS";
const DEFAULT_ORIGIN_ADDRESS: &str = "http://127.0.0.1:8545";
const DEFAULT_AUXILIARY_ADDRESS: &str = "http://127.0.0.1:8546";

/// Global config for running a mosaic node.
pub struct Config {
    /// Address of the origin chain, e.g. "127.0.0.1:8485"
    origin_address: String,
    /// Address of the auxiliary chain, e.g. "127.0.0.1:8486"
    auxiliary_address: String,
}

impl Config {
    /// Reads the configuration from environment variables and creates a new Config from them. In
    /// case an environment variable is not set, a default fallback will be used.
    pub fn new() -> Result<Config, &'static str> {
        // Read origin address from env and set it or fallback to default
        let origin_address = env::var(ENV_ORIGIN_ADDRESS);
        let origin_address = match origin_address {
            Ok(address) => address,
            Err(_) => {
                info!("No origin chain address given, falling back to default.");
                DEFAULT_ORIGIN_ADDRESS.to_string()
            }
        };

        // Read auxiliary address from env and set it or fallback to default
        let auxiliary_address = env::var(ENV_AUXILIARY_ADDRESS);
        let auxiliary_address = match auxiliary_address {
            Ok(address) => address,
            Err(_) => {
                info!("No auxiliary chain address given, falling back to default.");
                DEFAULT_AUXILIARY_ADDRESS.to_string()
            }
        };

        info!("Using origin chain address: {}", origin_address);
        info!("Using auxiliary chain address: {}", auxiliary_address);

        Ok(Config {
            origin_address,
            auxiliary_address,
        })
    }
}

/// Runs a mosaic node with the given configuration.
/// Prints all accounts of the origin blockchain to std out.
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let ethereum = blockchain::connect_to_ethereum(config.origin_address);
    let accounts = ethereum.get_accounts();

    println!("Accounts:");
    for account in accounts {
        println!("{}", account);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn the_config_reads_the_environment_variables() {
        let config = Config::new().unwrap();
        assert_eq!(config.origin_address, DEFAULT_ORIGIN_ADDRESS.to_string());
        assert_eq!(config.auxiliary_address, DEFAULT_AUXILIARY_ADDRESS.to_string());

        env::set_var(ENV_ORIGIN_ADDRESS, "10.0.0.1");
        let config = Config::new().unwrap();
        assert_eq!(config.origin_address, "10.0.0.1");
        assert_eq!(config.auxiliary_address, DEFAULT_AUXILIARY_ADDRESS.to_string());

        env::set_var(ENV_AUXILIARY_ADDRESS, "10.0.0.2");
        let config = Config::new().unwrap();
        assert_eq!(config.origin_address, "10.0.0.1");
        assert_eq!(config.auxiliary_address, "10.0.0.2");

        env::remove_var(ENV_ORIGIN_ADDRESS);
        env::remove_var(ENV_AUXILIARY_ADDRESS);
    }
}
