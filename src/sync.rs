use anyhow::Result;
use skyeng_words::client::*;
use skyeng_words::db;
use std::collections::HashSet;
use std::future::Future;

pub async fn sync(client: &Client) -> Result<()> {
    log::info!("start fetching wordsets");
    let wordsets = get_wordsets(client).await?;
    log::info!("got {} wordsets", wordsets.len());

    log::info!("start fetching words");
    let mut words = Vec::with_capacity(wordsets.len() * 10);
    for ws in wordsets {
        words.extend(fetch_until_completed(|ps, p| client.words_of_wordset(ws.id, ps, p)).await?);
    }
    log::info!("got {} words", words.len());

    log::info!("start fetching meanings");
    let mut meanings = Vec::with_capacity(words.len());
    for chunk in words.chunks(30) {
        meanings.extend(client.meanings(&(chunk.iter().map(|w| w.meaning_id.to_string()).collect::<Vec<String>>())).await?);
    }
    log::info!("got {} meanings", meanings.len());

    let filtered: HashSet<i32> =
        HashSet::from_iter(db::filter_ids(&meanings.iter().map(|m| m.id).collect()).await?);
    log::info!("got {} new meanings", filtered.len());

    log::info!("start saving meanings to db");
    let meanings: Vec<Meaning> = meanings
        .into_iter()
        .filter(|p| filtered.contains(&p.id))
        .collect();
    db::save_new_words(meanings).await?;
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
