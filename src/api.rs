#![allow(dead_code)]

use serde::Deserialize;

pub const RAPID_API_TRANSLATE: &str = "https://text-translator2.p.rapidapi.com/translate";

pub const FLAGS_API: &str =
    "https://restcountries.com/v3.1/all?fields=name,flags,translations,coatOfArms";

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

pub type Countries = Vec<Country>;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub name: CountryName,
    pub flags: CountryImages,
    pub coat_of_arms: CountryImages,
    pub translations: Translation,
}

#[derive(Deserialize, Clone)]
pub struct CountryName {
    pub common: String,
    pub official: String,
}

#[derive(Deserialize, Clone)]
pub struct Translation {
    pub spa: CountryName,
}

#[derive(Deserialize, Clone)]
pub struct CountryImages {
    pub png: String,
}
