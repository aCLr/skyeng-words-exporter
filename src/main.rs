use std::fs::File;
use std::io::prelude::Write;

use anyhow::Result;


use client::Client;
use std::env;

mod client;
mod models;
mod error;

const PAGE_SIZE: i32 = 999;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(
        env::var("LOGIN").expect("login expected"),
        env::var("PASSWORD").expect("password expected"),
    )?;

    client.login().await?;

    let wordsets = client.wordsets_page(PAGE_SIZE, 1).await?;
    //
    // let wwws: Vec<WordsResp> = wordsets
    //     .data
    //     .iter()
    //     .map(|ws|client.words_of_wordset(ws.id, PAGE_SIZE, 1))
    //     .collect();
    // println!("{}", wwws.len());
    //
    // let result: Vec<Meaning> = wwws
    //     .iter()
    //     .map(|www| async {
    //         client.meanings(
    //             &www.data
    //                 .iter()
    //                 .map(|w| w.meaningId.to_string())
    //                 .collect::<Vec<String>>(),
    //         ).await
    //     })
    //     .flatten()
    //     .flatten()
    //     .collect();
    //
    // let mut file = File::create("result.json")?;
    // file.write_all(serde_json::to_string(&result)?.as_bytes())?;

    Ok(())
}
