use sea_orm::Statement;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::ConnectionTrait;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table_words"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
        CREATE TABLE words (
            id int NOT NULL PRIMARY KEY,
            word_id int not null,
            difficulty_level int not null,
            text text not null,
            translation text not null,
            definition text not null,
            is_gold_3000 boolean not null,
            examples text not null
        )"#;
        let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
        manager.get_connection().execute(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            "drop table words".to_string(),
        );
        manager.get_connection().execute(stmt).await.map(|_| ())
    }
}
