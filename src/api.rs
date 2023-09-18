#![allow(dead_code)]

use serde::Deserialize;

pub const RAPID_API_TRANSLATE: &str = "https://text-translator2.p.rapidapi.com/translate";

pub const MEME_API: &str = "https://meme-api.com/gimme/{subreddit}/{count}";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translator {
    pub data: TranslatorData,
    status: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatorData {
    pub translated_text: String,
}

type Link = String;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meme {
    pub post_link: Link,
    pub subreddit: String,
    pub title: String,
    pub url: Link,
    pub nsfw: bool,
    pub spoiler: bool,
    pub author: String,
    pub ups: i32,
    pub preview: Vec<Link>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memes {
    pub count: i64,
    pub memes: Vec<Meme>,
}
