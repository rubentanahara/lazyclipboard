use rusqlite::Connection;

const MIGRATIONS: [&str; 1] = [include_str!("migrations/0001_schema_v1.sql")];

pub fn migrate(connection: &mut Connection) -> rusqlite::Result<()> {
    let applied: i64 = connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    let transaction = connection.transaction()?;
    for (index, migration) in MIGRATIONS.iter().enumerate().skip(applied as usize) {
        transaction.execute_batch(migration)?;
        transaction.pragma_update(None, "user_version", index as i64 + 1)?;
    }
    transaction.commit()
}

pub mod seed;

#[cfg(test)]
mod tests;
