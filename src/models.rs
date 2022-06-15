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
}

#[derive(Debug, Deserialize)]
pub struct WordsetsResp {
    pub meta: MetaResp,
    pub data: Vec<Wordset>,
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
    text: String,
    translation: TextFieldOnly,
    definition: TextFieldOnly,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextFieldOnly {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct DefaultWordset {
    id: i32,
    #[serde(rename(deserialize="countOfWords"))]
    count_of_words: i32
}