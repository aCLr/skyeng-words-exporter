use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(untagged)]
pub enum LoginResp {
    Success {
        redirect: String,
        success: bool,
    },
    Failed {
        message: String,
        code: String,
        success: bool,
    },
}

#[derive(Deserialize)]
pub struct Users {
    pub users: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct MetaResp {
    pub total: i32,
    #[serde(rename(deserialize = "currentPage"))]
    pub current_page: i32,
    #[serde(rename(deserialize = "lastPage"))]
    pub last_page: i32,
    #[serde(rename(deserialize = "pageSize"))]
    pub page_size: i32,
}

#[derive(Debug, Deserialize)]
pub struct Wordset {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct WordsetsResp {
    meta: MetaResp,
    data: Vec<Wordset>,
}

#[derive(Debug, Deserialize)]
pub struct Word {
    #[serde(rename(deserialize = "meaningId"))]
    pub meaning_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct WordsResp {
    meta: MetaResp,
    data: Vec<Word>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meaning {
    pub id: i32,
    #[serde(rename(deserialize = "wordId"))]
    pub word_id: i32,
    #[serde(rename(deserialize = "difficultyLevel"))]
    pub difficulty_level: Option<i8>,
    pub text: String,
    pub translation: TextFieldOnly,
    pub definition: Option<TextFieldOnly>,
    #[serde(rename(deserialize = "isGold3000"))]
    pub is_gold_3000: bool,
    pub examples: Vec<TextFieldOnly>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextFieldOnly {
    pub text: String,
}

#[derive(Deserialize)]
pub struct DefaultWordset {
    id: i32,
    title: String,
    // #[serde(rename(deserialize = "countOfWords"))]
    // count_of_words: i32,
}

impl From<DefaultWordset> for Wordset {
    fn from(w: DefaultWordset) -> Self {
        Self {
            id: w.id,
            title: w.title,
        }
    }
}

pub trait Resp<D> {
    fn get_meta(&self) -> &MetaResp;
    fn get_data(self) -> Vec<D>;
}

impl Resp<Word> for WordsResp {
    fn get_meta(&self) -> &MetaResp {
        &self.meta
    }

    fn get_data(self) -> Vec<Word> {
        self.data
    }
}

impl Resp<Wordset> for WordsetsResp {
    fn get_meta(&self) -> &MetaResp {
        &self.meta
    }

    fn get_data(self) -> Vec<Wordset> {
        self.data
    }
}
