pub mod client;
pub mod consts;
pub mod errors;
pub mod lang;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod client_tests {
    use crate::client::ClientBuilder;
    use crate::consts::RUNTIMES_URL;
    use crate::lang::Language;

    use super::*;

    #[tokio::test]
    async fn creating_client() {
        let builder = ClientBuilder::new()
            .set_lang("rs")
            .set_main_file("fn main() { println!(\"Hello, world!\") }")
            .build()
            .map_err(|err| println!("{}", err.as_str()));

        assert!(matches!(builder, Ok(_)));
    }

    #[tokio::test]
    async fn non_passing_main_file() {
        let builder = ClientBuilder::new().set_lang("rs").build();

        assert!(matches!(builder, Err(_)));
    }

    #[tokio::test]
    async fn non_passing_lang() {
        let builder = ClientBuilder::new()
            .set_main_file("fn main() { println!(\"Hello, world!\") }")
            .build();

        assert!(matches!(builder, Err(_)));
    }

    #[tokio::test]
    async fn sending_post() {
        let client = ClientBuilder::new()
            .set_lang("rust")
            .set_main_file("fn main() { println!(\"Hello, world!\") }")
            .build()
            .unwrap();

        let result = client
            .execute()
            .await
            .map_err(|err| println!("{}", err.as_str()));

        assert!(matches!(result, Ok(_)));
    }
}
