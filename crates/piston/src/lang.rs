use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Language {
    pub(crate) language: String,
    pub(crate) version: String,
    pub(crate) aliases: AliasesLang,
    pub(crate) runtime: Option<String>,
}

type AliasesLang = Vec<String>;

#[derive(Debug, Deserialize)]
pub struct Response {
    language: String,
    version: String,
    run: RunData,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct RunData {
    stdout: String,
    output: String,
    code: u8,
    signal: Option<String>,
}
