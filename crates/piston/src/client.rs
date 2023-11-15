use serde::{Deserialize, Serialize};

use crate::{
    consts::{EXECUTE_URL, RUNTIMES_URL},
    errors::Errors,
    lang::{Language, Response},
};

#[derive(Serialize)]
struct Data {
    language: String,
    version: String,
    files: Vec<FileData>,
}

#[derive(Serialize)]
struct FileData {
    name: Option<String>,
    content: String,
}

pub struct Client {
    language: String,
    main_file: String,
    add_files: Vec<String>,
    client: reqwest::Client,
}

impl Client {
    async fn get_lang_version(&self) -> Result<Option<String>, reqwest::Error> {
        let result = self.client.get(RUNTIMES_URL).send().await?;
        let json = result.json::<Vec<Language>>().await?;
        Ok(json
            .iter()
            .find(|lang| {
                (lang.language == self.language.to_owned())
                    | (lang.aliases.contains(&self.language))
            })
            .map_or(None, |l| Some(l.version.clone())))
    }

    pub async fn execute(self) -> Result<Response, Errors> {
        let version = match self.get_lang_version().await {
            Err(err) => return Err(Errors::Unknown(err.to_string())),
            Ok(None) => return Err(Errors::InvalidLanguage),
            Ok(Some(v)) => v,
        };
        let mut map_files = vec![FileData {
            name: None,
            content: self.main_file,
        }];
        map_files.extend(
            self.add_files
                .iter()
                .map(|code| FileData {
                    name: None,
                    content: code.to_owned(),
                })
                .collect::<Vec<_>>(),
        );

        let data = self
            .client
            .post(EXECUTE_URL)
            .json(&Data {
                language: self.language,
                version,
                files: map_files,
            })
            .send()
            .await;
        let Ok(data) = data else {
            return Err(Errors::BadRequest);
        };
        data.json::<Response>()
            .await
            .map_err(|err| Errors::Unknown(err.to_string()))
    }
}

pub struct ClientBuilder {
    language: Option<String>,
    main_file: Option<String>,
    add_files: Vec<String>,
}

impl ClientBuilder {
    pub fn new() -> ClientBuilder {
        Self {
            language: None,
            main_file: None,
            add_files: vec![],
        }
    }

    pub fn set_lang(self, lang: &str) -> ClientBuilder {
        ClientBuilder {
            language: Some(lang.to_owned()),
            ..self
        }
    }

    pub fn set_main_file(self, code: &str) -> ClientBuilder {
        ClientBuilder {
            main_file: Some(code.to_owned()),
            ..self
        }
    }

    pub fn add_files(self, files: Vec<&str>) -> ClientBuilder {
        ClientBuilder {
            add_files: files.iter().map(|s| s.to_string()).collect(),
            ..self
        }
    }

    pub fn build(self) -> Result<Client, Errors> {
        let language = self.language.ok_or(Errors::MissingLang)?;
        let main_file = self.main_file.ok_or(Errors::MissingCode)?;

        let http_client = reqwest::ClientBuilder::new()
            .user_agent("fewwis-bot/@romancitodev")
            .build()
            .unwrap();
        Ok(Client {
            language,
            main_file,
            add_files: self.add_files,
            client: http_client,
        })
    }
}
