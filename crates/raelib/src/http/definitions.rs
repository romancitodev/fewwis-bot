#![allow(dead_code)]
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    word: String,
    variant: Option<String>,
    definitions: Vec<Definition>,
}

impl Word {
    pub fn new(word: String, variant: Option<String>, definitions: Vec<Definition>) -> Self {
        Self {
            word,
            variant,
            definitions,
        }
    }

    pub fn word(&self) -> &str {
        self.word.as_ref()
    }

    pub fn variant(&self) -> Option<&String> {
        self.variant.as_ref()
    }

    pub fn definitions(&self) -> &[Definition] {
        self.definitions.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    def_type: String,
    definition: String,
}

impl Definition {
    pub fn new(def_type: String, definition: String) -> Self {
        Self {
            def_type,
            definition,
        }
    }

    pub fn def_type(&self) -> &str {
        self.def_type.as_ref()
    }

    pub fn definition(&self) -> &str {
        self.definition.as_ref()
    }
}

#[derive(Deserialize)]
pub(crate) struct WordMetaData {
    approx: u32,
    res: Vec<WordResData>,
}

impl WordMetaData {
    pub fn approx(&self) -> u32 {
        self.approx
    }

    pub fn res(&self) -> &[WordResData] {
        self.res.as_ref()
    }
}

#[derive(Deserialize)]
pub struct WordResData {
    header: String,
    id: String,
    grp: u32,
}

impl WordResData {
    pub fn header(&self) -> &str {
        self.header.as_ref()
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn grp(&self) -> u32 {
        self.grp
    }
}
