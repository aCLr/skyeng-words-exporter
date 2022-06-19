pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_words;
mod m20220619_130453_create_wordset;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_words::Migration),
            Box::new(m20220619_130453_create_wordset::Migration),
        ]
    }
}
