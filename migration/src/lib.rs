pub use sea_orm_migration::prelude::*;

mod m20241010_162704_create_table_todos;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20241010_162704_create_table_todos::Migration)]
    }
}
