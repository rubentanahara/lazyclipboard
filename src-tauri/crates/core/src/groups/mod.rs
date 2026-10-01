use crate::model::{CommandError, Group, GroupId};
use rusqlite::{ffi, params, Connection, Row};

const NAME_FIELD: &str = "name";

const INSERT_AT_END: &str = "INSERT INTO groups (name, position, created_at) \
     VALUES (?1, (SELECT COALESCE(MAX(position), -1) + 1 FROM groups), ?2) \
     RETURNING id, name, position, never_send_to_ai, created_at";

const SELECT_SUMMARIES: &str =
    "SELECT g.id, g.name, g.position, g.never_send_to_ai, g.created_at, \
     count(i.id) AS item_count, max(i.captured_at) AS last_activity \
     FROM groups g LEFT JOIN items i ON i.group_id = g.id AND i.deleted_at IS NULL \
     GROUP BY g.id ORDER BY g.position, g.id";

#[derive(Debug, Clone, PartialEq)]
pub struct GroupSummary {
    pub group: Group,
    pub item_count: u32,
    pub last_activity: Option<i64>,
}

pub fn list(connection: &Connection) -> Result<Vec<GroupSummary>, CommandError> {
    summaries(connection).map_err(command_error)
}

pub fn create(connection: &Connection, name: &str, created_at: i64) -> Result<Group, CommandError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(CommandError::Validation {
            field: NAME_FIELD.to_owned(),
        });
    }
    connection
        .prepare_cached(INSERT_AT_END)
        .and_then(|mut statement| statement.query_row(params![name, created_at], group_from_row))
        .map_err(command_error)
}

fn command_error(error: rusqlite::Error) -> CommandError {
    match error {
        rusqlite::Error::SqliteFailure(failure, _)
            if failure.extended_code == ffi::SQLITE_CONSTRAINT_UNIQUE =>
        {
            CommandError::Conflict
        }
        _ => CommandError::Internal,
    }
}

fn summaries(connection: &Connection) -> rusqlite::Result<Vec<GroupSummary>> {
    let mut statement = connection.prepare_cached(SELECT_SUMMARIES)?;
    let rows = statement.query_map([], summary_from_row)?;
    rows.collect()
}

fn summary_from_row(row: &Row) -> rusqlite::Result<GroupSummary> {
    Ok(GroupSummary {
        group: group_from_row(row)?,
        item_count: row.get("item_count")?,
        last_activity: row.get("last_activity")?,
    })
}

fn group_from_row(row: &Row) -> rusqlite::Result<Group> {
    Ok(Group {
        id: GroupId(row.get("id")?),
        name: row.get("name")?,
        position: row.get("position")?,
        never_send_to_ai: row.get("never_send_to_ai")?,
        created_at: row.get("created_at")?,
    })
}

#[cfg(test)]
mod tests;
