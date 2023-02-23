use serde_derive::Deserialize;

use crate::provider::Provider;

/**
 The config file
*/
#[derive(Deserialize)]
pub struct Config {
    pub providers: Vec<Provider>,
}

