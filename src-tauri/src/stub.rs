use std::sync::{Mutex, MutexGuard};

use lazyclipboard_core::db::{migrate, seed};
use lazyclipboard_core::model::{CommandError, Group, GroupId, ItemContent, ItemId, ItemPreview};
use rusqlite::{params, Connection, OptionalExtension, Row};

const PREVIEW_COLUMNS: &str = "id, COALESCE(plain_text, ''), html IS NOT NULL, image_file";
const NEWEST_FIRST: &str = "deleted_at IS NULL ORDER BY captured_at DESC, id DESC";
const IMAGE_URL_PREFIX: &str = "asset://localhost/images/";

pub struct Stub {
    connection: Mutex<Connection>,
}

impl Stub {
    pub fn seeded() -> rusqlite::Result<Self> {
        let mut connection = Connection::open_in_memory()?;
        migrate(&mut connection)?;
        seed::load(&mut connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn groups(&self) -> Result<Vec<Group>, CommandError> {
        let connection = self.connection()?;
        let mut statement = connection
            .prepare("SELECT id, name, position, never_send_to_ai, created_at FROM groups ORDER BY position")
            .map_err(internal)?;
        let groups = statement
            .query_map([], group_from_row)
            .map_err(internal)?
            .collect::<Result<_, _>>()
            .map_err(internal)?;
        Ok(groups)
    }

    pub fn group(&self, id: GroupId) -> Result<Group, CommandError> {
        self.connection()?
            .query_row(
                "SELECT id, name, position, never_send_to_ai, created_at FROM groups WHERE id = ?1",
                [id.0],
                group_from_row,
            )
            .optional()
            .map_err(internal)?
            .ok_or(CommandError::NotFound)
    }

    pub fn items_in_group(&self, group_id: GroupId) -> Result<Vec<ItemPreview>, CommandError> {
        self.previews(
            &format!("SELECT {PREVIEW_COLUMNS} FROM items WHERE group_id = ?1 AND {NEWEST_FIRST}"),
            params![group_id.0],
        )
    }

    pub fn items_matching(&self, query: &str) -> Result<Vec<ItemPreview>, CommandError> {
        self.previews(
            &format!("SELECT {PREVIEW_COLUMNS} FROM items WHERE instr(lower(plain_text), lower(?1)) > 0 AND {NEWEST_FIRST}"),
            params![query],
        )
    }

    pub fn item(&self, id: ItemId) -> Result<ItemContent, CommandError> {
        self.connection()?
            .query_row(
                "SELECT kind, COALESCE(plain_text, ''), image_file, image_width, image_height FROM items WHERE id = ?1 AND deleted_at IS NULL",
                [id.0],
                content_from_row,
            )
            .optional()
            .map_err(internal)?
            .ok_or(CommandError::NotFound)
    }

    fn previews(
        &self,
        sql: &str,
        parameters: impl rusqlite::Params,
    ) -> Result<Vec<ItemPreview>, CommandError> {
        let connection = self.connection()?;
        let mut statement = connection.prepare(sql).map_err(internal)?;
        let previews = statement
            .query_map(parameters, preview_from_row)
            .map_err(internal)?
            .collect::<Result<_, _>>()
            .map_err(internal)?;
        Ok(previews)
    }

    fn connection(&self) -> Result<MutexGuard<'_, Connection>, CommandError> {
        self.connection.lock().map_err(|_| CommandError::Internal)
    }
}

fn internal(_: rusqlite::Error) -> CommandError {
    CommandError::Internal
}

fn group_from_row(row: &Row) -> rusqlite::Result<Group> {
    Ok(Group {
        id: GroupId(row.get(0)?),
        name: row.get(1)?,
        position: row.get(2)?,
        never_send_to_ai: row.get(3)?,
        created_at: row.get(4)?,
    })
}

fn preview_from_row(row: &Row) -> rusqlite::Result<ItemPreview> {
    let image_file: Option<String> = row.get(3)?;
    Ok(ItemPreview {
        id: ItemId(row.get(0)?),
        plain_text: row.get(1)?,
        has_rich_text: row.get(2)?,
        image_url: image_file.map(|file| format!("{IMAGE_URL_PREFIX}{file}")),
    })
}

fn content_from_row(row: &Row) -> rusqlite::Result<ItemContent> {
    let kind: String = row.get(0)?;
    let plain_text: String = row.get(1)?;
    Ok(match kind.as_str() {
        "link" => ItemContent::Link { url: plain_text },
        "image" => ItemContent::Image {
            file: row.get(2)?,
            width: row.get(3)?,
            height: row.get(4)?,
        },
        _ => ItemContent::Text { text: plain_text },
    })
}
