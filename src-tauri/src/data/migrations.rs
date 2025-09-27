use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration> {
    let mut migrations: Vec<Migration> = vec![];

    // Create Card table
    //
    // quiz will be a serialized json text
    migrations.push(
        Migration {
            version: 1,
            description: "Card table creation",
            sql: "
                CREATE TABLE cards (
                        id INTEGER PRIMARY KEY, 
                        title TEXT, 
                        content TEXT,
                        quiz TEXT);
                ",
            kind: MigrationKind::Up,
        }
    );
    migrations
}
