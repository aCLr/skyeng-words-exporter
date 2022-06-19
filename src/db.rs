use crate::client::models::{Meaning, Wordset};
use anyhow::{bail, Result};
use entity::{words, wordsets};
use once_cell::sync::OnceCell;
use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection,
    EntityTrait, QueryFilter, QueryResult, Statement,
};

static POOL: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init_pool(db_url: &str) {
    let mut opt = ConnectOptions::from(db_url);
    opt.max_connections(1).min_connections(1);
    POOL.set(
        Database::connect(opt)
            .await
            .expect("can't initialize database pool"),
    )
    .expect("pool already initialized");
}

fn get_pool() -> &'static DatabaseConnection {
    POOL.get().expect("db pool not initialized yet")
}

pub async fn save_new_ws_words(meanings: Vec<Meaning>, wordset_id: i32) -> Result<()> {
    words::Entity::insert_many(
        meanings
            .into_iter()
            .map(|m| make_word(m, wordset_id))
            .collect::<Vec<words::ActiveModel>>(),
    )
    .exec(get_pool())
    .await?;
    Ok(())
}

pub async fn save_ws_if_not_exists(wordset: &Wordset) -> Result<()> {
    if let Some(_) = wordsets::Entity::find_by_id(wordset.id)
        .one(get_pool())
        .await?
    {
        return Ok(());
    };
    wordsets::Entity::insert(wordsets::ActiveModel {
        id: Set(wordset.id),
        name: Set(wordset.title.to_owned()),
    })
    .exec(get_pool())
    .await?;
    Ok(())
}

fn make_word(mean: Meaning, wordset_id: i32) -> words::ActiveModel {
    words::ActiveModel {
        id: Set(mean.id),
        word_id: Set(mean.word_id),
        difficulty_level: Set(mean.difficulty_level.unwrap_or_default().into()),
        text: Set(mean.text),
        translation: Set(mean.translation.text),
        definition: Set(mean.definition.map_or("".to_string(), |t| t.text)),
        is_gold_3000: Set(mean.is_gold_3000),
        examples: Set(mean
            .examples
            .into_iter()
            .map(|t| t.text)
            .collect::<Vec<String>>()
            .join(",")),
        wordset_id: Set(wordset_id),
        exported: Set(false),
    }
}

pub async fn mark_as_exported(ids: Vec<i32>) -> Result<()> {
    words::Entity::update_many()
        .set(words::ActiveModel {
            id: NotSet,
            word_id: NotSet,
            difficulty_level: NotSet,
            text: NotSet,
            translation: NotSet,
            definition: NotSet,
            is_gold_3000: NotSet,
            examples: NotSet,
            wordset_id: NotSet,
            exported: Set(true),
        })
        .filter(words::Column::Id.is_in(ids))
        .exec(get_pool())
        .await?;
    Ok(())
}

pub async fn get_unexported_words() -> Result<Vec<words::Model>> {
    Ok(words::Entity::find()
        .filter(words::Column::Exported.eq(false))
        .all(get_pool())
        .await?)
}

pub async fn get_all_words() -> Result<Vec<words::Model>> {
    Ok(words::Entity::find().all(get_pool()).await?)
}

pub async fn get_ws_id_by_name(name: String) -> Result<i32> {
    Ok(
        match wordsets::Entity::find()
            .filter(wordsets::Column::Name.eq(name))
            .one(get_pool())
            .await?
        {
            Some(ws) => ws.id,
            None => bail!("wordset not found"),
        },
    )
}

pub async fn filter_unexisted_word_ids(found: &Vec<i32>) -> Result<Vec<i32>> {
    let query_res: Vec<QueryResult> = get_pool()
        .query_all(Statement::from_string(
            DatabaseBackend::Sqlite,
            format!(
                r#"
            with cte(id) as
                 (values {values})
            select id
            from cte
            where id not in (select id from words);
            "#,
                values = found
                    .iter()
                    .map(|v| format!("({v})"))
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .to_owned(),
        ))
        .await?;
    let mut res = Vec::with_capacity(query_res.len());
    for row in query_res.into_iter() {
        res.push(row.try_get("", "id")?);
    }

    Ok(res)
}
