use crate::client::*;
use crate::db;
use anyhow::Result;
use std::collections::HashSet;
use std::future::Future;

pub async fn sync(client: &Client) -> Result<()> {
    log::info!("start fetching wordsets");
    let wordsets = get_wordsets(client).await?;
    log::info!("got {} wordsets", wordsets.len());

    for (i, ws) in wordsets.into_iter().enumerate() {
        log::info!("{num} wordset started", num = i + 1);
        db::save_ws_if_not_exists(&ws).await?;
        sync_wordset(client, IdOrName::Id(ws.id)).await?;
    }

    Ok(())
}

pub enum IdOrName {
    Id(i32),
    Name(String),
}

pub async fn sync_wordset(client: &Client, ws_id_or_name: IdOrName) -> Result<()> {
    let ws_id = match ws_id_or_name {
        IdOrName::Id(id) => id,
        IdOrName::Name(name) => db::get_ws_id_by_name(name).await?,
    };
    log::info!("start fetching words");
    let words = fetch_until_completed(|ps, p| client.words_of_wordset(ws_id, ps, p)).await?;
    log::info!("got {} words", words.len());

    log::info!("start fetching meanings");
    let meanings = client
        .meanings(
            &(words
                .iter()
                .map(|w| w.meaning_id.to_string())
                .collect::<Vec<String>>()),
        )
        .await?;
    log::info!("got {} meanings", meanings.len());

    let filtered: HashSet<i32> = HashSet::from_iter(
        db::filter_unexisted_word_ids(&meanings.iter().map(|m| m.id).collect()).await?,
    );
    log::info!("got {} new meanings", filtered.len());

    log::info!("start saving meanings to db");
    let meanings: Vec<Meaning> = meanings
        .into_iter()
        .filter(|p| filtered.contains(&p.id))
        .collect();
    if meanings.len() > 0 {
        db::save_new_ws_words(meanings, ws_id).await?;
    }
    Ok(())
}

async fn get_wordsets(client: &Client) -> Result<Vec<Wordset>> {
    let mut wordsets = fetch_until_completed(|ps, p| client.wordsets_page(ps, p)).await?;
    wordsets.push(client.default_wordset().await?.into());
    Ok(wordsets)
}

async fn fetch_until_completed<T, F, R, Fut>(call: F) -> Result<Vec<R>>
where
    T: Resp<R>,
    F: Fn(i32, i32) -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut resp = call(100, 1).await?;
    let total = resp.get_meta().total as usize;
    let mut result: Vec<R> = resp.get_data();
    while total as usize > result.len() {
        resp = call(100, 1).await?;
        result.extend(resp.get_data())
    }

    Ok(result)
}
