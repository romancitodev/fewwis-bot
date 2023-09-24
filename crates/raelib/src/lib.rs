#![allow(dead_code)]
mod consts;
mod http;
mod prelude;
mod utils;

#[cfg(test)]
mod tests {
    use prelude::client::RaeClient;

    use super::*;

    #[tokio::test]
    async fn test_get_endpoint() {
        let client = RaeClient::new();
        let defs = client.get_definitions("norcoreano").await;
        assert!(defs.is_ok());
    }

    #[tokio::test]
    async fn test_multiple_attrs() {
        let client = RaeClient::new();
        let defs = client.get_definitions("mineroducto").await;
        assert!(defs.is_ok());
    }

    #[tokio::test]
    async fn test_get_random() {
        let client = RaeClient::new();
        for _ in 0..5 {
            let defs = client.get_random().await;
            assert!(defs.is_ok());
            println!("{:#?}", defs.unwrap());
        }
    }
}
