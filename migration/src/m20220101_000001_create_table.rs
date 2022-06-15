use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt(r#"
        CREATE TABLE words (
            id INTEGER PRIMARY KEY,
            difficulty_level INTEGER,
            text TEXT NOT NULL,
            translation TEXT NOT NULL,
            definition TEXT NOT NULL,
            is_gold_3000 BOOLEAN,
            examples TEXT
        )"#).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.exec_stmt("DROP TABLE words").await
    }
}
