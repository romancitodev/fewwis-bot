#[derive(Debug)]
pub enum Errors {
    UnknownLang,
    MissingLang,
    MissingCode,
    InvalidLanguage,
    BadRequest,
    Unknown(String),
}

impl Errors {
    pub fn as_str(&self) -> &str {
        match self {
            Self::UnknownLang => "Unknown language",
            Self::MissingLang => "Missing language",
            Self::MissingCode => "Missing main file",
            Self::InvalidLanguage => "Invalid language, see the API instead.",
            Self::BadRequest => "Bad request",
            Self::Unknown(err) => &err,
        }
    }
}
