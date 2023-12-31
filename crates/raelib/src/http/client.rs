use std::ops::Range;

use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};

use crate::utils;

use super::definitions::{Word, WordMetaData};

pub type Error = crate::http::errors::Error;

#[derive(Debug, Clone)]
pub struct RaeClient {
    client: reqwest::Client,
}

impl Default for RaeClient {
    fn default() -> Self {
        let client = reqwest::Client::builder()
            .user_agent(crate::consts::USER_AGENT)
            .default_headers(HeaderMap::from_iter([
                (AUTHORIZATION, HeaderValue::from_static(crate::consts::AUTH)),
                (
                    CONTENT_TYPE,
                    HeaderValue::from_static(crate::consts::CONTENT_TYPE),
                ),
                (
                    USER_AGENT,
                    HeaderValue::from_static(crate::consts::USER_AGENT),
                ),
            ]))
            .build()
            .unwrap();
        RaeClient { client }
    }
}

impl RaeClient {
    pub async fn get_definitions(&self, word: &str) -> Result<Word, Error> {
        let word_id = self.search_word(word).await?;
        let request = self
            .client
            .get(format!("{}fetch?id={}", crate::consts::BASE_URL, word_id))
            .send()
            .await?
            .text()
            .await?;
        let request = utils::parse_request(request);
        Ok(request)
    }

    pub async fn get_random(&self) -> Result<Word, Error> {
        let request = self
            .client
            .get(format!("{}random", crate::consts::BASE_URL))
            .send()
            .await?
            .text()
            .await?;
        let request = utils::parse_request(request);
        Ok(request)
    }

    pub async fn get_random_with_range(&self, range: Range<i32>) -> Result<Vec<Word>, Error> {
        let mut requests = vec![];
        for _ in range {
            let request = self
                .client
                .get(format!("{}random", crate::consts::BASE_URL))
                .send()
                .await?
                .text()
                .await?;
            let request = utils::parse_request(request);
            requests.push(request)
        }
        Ok(requests)
    }

    async fn search_word(&self, word: &str) -> Result<String, Error> {
        let request = self
            .client
            .get(format!("{}search?w={}", crate::consts::BASE_URL, word))
            .send()
            .await?
            .json::<WordMetaData>()
            .await?;
        Ok(request.res()[0].id().to_string())
    }
}
