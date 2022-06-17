use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use reqwest::IntoUrl;
use std::collections::HashMap;

use super::models::*;
use crate::error::*;
use reqwest::{
    Client as ReqClient, ClientBuilder as ReqClientBuilder, RequestBuilder as ReqRequestBuilder,
};
use scraper::{Html, Selector};

struct Credentials {
    token: OnceCell<String>,
    login: String,
    password: String,
    user_id: OnceCell<i32>,
}

impl Credentials {
    fn new(login: String, password: String) -> Self {
        Self {
            login,
            password,
            token: OnceCell::new(),
            user_id: OnceCell::new(),
        }
    }
    pub fn token(&self) -> &str {
        self.token.get().expect("token not set yet").as_str()
    }
    pub fn user_id(&self) -> &i32 {
        self.user_id.get().expect("user_id not set yet")
    }
    pub fn set_token(&self, token: String) {
        self.token.set(token).expect("token already set");
    }
    pub fn set_user_id(&self, user_id: i32) {
        self.user_id.set(user_id).expect("user_id already set");
    }
}

pub struct Client {
    inner: ReqClient,
    creds: Credentials,
}

impl Client {
    pub fn new(login: String, password: String) -> Result<Client> {
        Ok(Client {
            inner: ReqClientBuilder::new().cookie_store(true).build()?,
            creds: Credentials::new(login, password),
        })
    }
    fn get<U: IntoUrl>(&self, url: U) -> ReqRequestBuilder {
        self.inner
            .get(url)
            .header("Authorization", format!("Bearer {}", self.creds.token()))
    }

    fn post<U: IntoUrl>(&self, url: U) -> ReqRequestBuilder {
        self.inner
            .post(url)
            .header("Authorization", format!("Bearer {}", self.creds.token()))
    }

    fn put<U: IntoUrl>(&self, url: U) -> ReqRequestBuilder {
        self.inner
            .put(url)
            .header("Authorization", format!("Bearer {}", self.creds.token()))
    }

    async fn get_user_id(&self) -> Result<i32> {
        let resp: Users = self
            .post("https://api-student.skyeng.ru/api/v2/users")
            .send()
            .await?
            .json()
            .await?;
        Ok(resp.users.first().unwrap().id)
    }

    pub async fn login(&self) -> Result<()> {
        let redirect_url: String = match self
            .inner
            .post("https://id.skyeng.ru/frame/login-submit")
            .form(&self.data_for_login().await?)
            .send()
            .await?
            .json::<LoginResp>()
            .await?
        {
            LoginResp::Success { redirect, success } => {
                assert!(success);
                redirect
            }
            LoginResp::Failed {
                message, success, ..
            } => {
                assert!(!success);
                bail!(message)
            }
        };

        self.get(redirect_url).send().await?;

        let jwt_resp = self
            .post("https://id.skyeng.ru/user-api/v1/auth/jwt")
            .send()
            .await?;
        match jwt_resp.headers().get("set-cookie") {
            None => {
                bail!("no set-cookie header")
            }
            Some(v) => {
                let cookie = v.to_str()?;
                let (tg, _) = cookie
                    .split_once(';')
                    .ok_or(Error::InvalidSkyengData("cookie not valid"))?;
                self.creds.set_token(
                    tg.strip_prefix("token_global=")
                        .ok_or(Error::InvalidSkyengData("jwt not found in cookies"))?
                        .to_string(),
                );
            }
        }

        self.creds.set_user_id(self.get_user_id().await?);

        Ok(())
    }

    async fn data_for_login(&self) -> Result<HashMap<String, String>> {
        let initial_resp = self
            .inner
            .get("https://id.skyeng.ru/login")
            .send()
            .await?
            .text()
            .await?;

        let doc = Html::parse_document(initial_resp.as_str());

        let form = doc
            .select(
                &Selector::parse("form.authentication-page__form[action=\"/frame/login-submit\"]")
                    .unwrap(),
            )
            .next()
            .unwrap();

        let mut data = HashMap::new();
        data.insert("username".to_string(), self.creds.login.clone());
        data.insert("password".to_string(), self.creds.password.clone());
        for node in form.children() {
            let val = node.value();
            if !val.is_element() {
                continue;
            }
            let inp = node.value().as_element().unwrap();
            if inp.name() != "input" {
                continue;
            }

            if inp.attr("type").unwrap() == "hidden" {
                data.insert(
                    inp.attr("name").unwrap().to_string(),
                    inp.attr("value").unwrap().to_string(),
                );
            }
        }

        Ok(data)
    }

    pub async fn default_wordset(&self) -> Result<DefaultWordset> {
        Ok(self
            .put("https://api-words.skyeng.ru/api/for-mobile/v1/wordsets/default.json")
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn wordsets_page(&self, page_size: i32, page: i32) -> Result<WordsetsResp> {
        Ok(self.get(
            format!("https://api-words.skyeng.ru/api/for-vimbox/v1/wordsets.json?studentId={student_id}&pageSize={page_size}&page={page}", student_id=self.creds.user_id())
        ).send().await?.json().await?)
    }

    pub async fn words_of_wordset(
        &self,
        wordset_id: i32,
        page_size: i32,
        page: i32,
    ) -> Result<WordsResp> {
        Ok(self
            .get(format!(
                r#"https://api-words.skyeng.ru/api/v1/wordsets/{wordset_id}/words.json?
    studentId={student_id}&
    wordsetId={wordset_id}&
    pageSize={page_size}&
    page={page}&
    acceptLanguage=ru"#,
                student_id = self.creds.user_id(),
            ))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn meanings(&self, meaning_ids: &Vec<String>) -> Result<Vec<Meaning>> {
        let joined = meaning_ids.join(",");
        Ok(self.get(format!(r#"https://dictionary.skyeng.ru/api/for-services/v2/meanings?ids={joined}&acceptLanguage=ru"#)).send().await?.json().await?)
    }
}
