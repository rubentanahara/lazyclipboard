use super::PendingItem;
use crate::model::{CommandError, GroupId, ItemId};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveRequest {
    pub group: GroupId,
    pub item: PendingItem,
    pub source_app: Option<String>,
    pub captured_at: i64,
}

struct Columns<'a> {
    kind: &'static str,
    plain_text: Option<&'a str>,
    html: Option<&'a str>,
    image: Option<ImageColumns<'a>>,
    byte_size: usize,
    content_hash: Vec<u8>,
}

struct ImageColumns<'a> {
    png: &'a [u8],
    width: u32,
    height: u32,
}

pub fn save(
    connection: &mut Connection,
    images_dir: &Path,
    request: SaveRequest,
) -> Result<ItemId, CommandError> {
    let columns = columns(&request.item);
    let transaction = connection.transaction().map_err(database_failure)?;
    ensure_group_exists(&transaction, request.group)?;
    let (row_id, new_image_path) =
        match find_duplicate(&transaction, request.group, &columns.content_hash)? {
            Some(row_id) => {
                move_to_top(&transaction, row_id, &columns, &request)?;
                (row_id, None)
            }
            None => insert(&transaction, images_dir, &columns, &request)?,
        };
    if let Err(error) = transaction.commit() {
        discard_orphan_image(new_image_path.as_deref());
        return Err(database_failure(error));
    }
    u32::try_from(row_id)
        .map(ItemId)
        .map_err(|_| CommandError::Internal)
}

fn columns(item: &PendingItem) -> Columns<'_> {
    match item {
        PendingItem::Text { text } => Columns {
            kind: "text",
            plain_text: Some(text),
            html: None,
            image: None,
            byte_size: text.len(),
            content_hash: content_hash("text", text.as_bytes()),
        },
        PendingItem::RichText { plain_text, html } => Columns {
            kind: "rich_text",
            plain_text: Some(plain_text),
            html: Some(html),
            image: None,
            byte_size: plain_text.len() + html.len(),
            content_hash: content_hash("rich_text", plain_text.as_bytes()),
        },
        PendingItem::Link { url } => Columns {
            kind: "link",
            plain_text: Some(url),
            html: None,
            image: None,
            byte_size: url.len(),
            content_hash: content_hash("link", url.as_bytes()),
        },
        PendingItem::Image { png, width, height } => Columns {
            kind: "image",
            plain_text: None,
            html: None,
            image: Some(ImageColumns {
                png,
                width: *width,
                height: *height,
            }),
            byte_size: png.len(),
            content_hash: content_hash("image", png),
        },
    }
}

fn content_hash(kind: &str, payload: &[u8]) -> Vec<u8> {
    Sha256::new()
        .chain_update(kind)
        .chain_update(payload)
        .finalize()
        .to_vec()
}

fn ensure_group_exists(transaction: &Transaction, group: GroupId) -> Result<(), CommandError> {
    transaction
        .query_row("SELECT id FROM groups WHERE id = ?1", [group.0], |_| Ok(()))
        .optional()
        .map_err(database_failure)?
        .ok_or(CommandError::NotFound)
}

fn find_duplicate(
    transaction: &Transaction,
    group: GroupId,
    content_hash: &[u8],
) -> Result<Option<i64>, CommandError> {
    transaction
        .query_row(
            "SELECT id FROM items WHERE group_id = ?1 AND content_hash = ?2 AND deleted_at IS NULL",
            params![group.0, content_hash],
            |row| row.get(0),
        )
        .optional()
        .map_err(database_failure)
}

fn insert(
    transaction: &Transaction,
    images_dir: &Path,
    columns: &Columns,
    request: &SaveRequest,
) -> Result<(i64, Option<PathBuf>), CommandError> {
    let image_path = columns
        .image
        .as_ref()
        .map(|image| write_image(images_dir, image.png))
        .transpose()?;
    let image_file = image_path
        .as_deref()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str());
    let inserted = transaction.execute(
        "INSERT INTO items (group_id, kind, plain_text, html, image_file, image_width, image_height, byte_size, content_hash, source_app, captured_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            request.group.0,
            columns.kind,
            columns.plain_text,
            columns.html,
            image_file,
            columns.image.as_ref().map(|image| image.width),
            columns.image.as_ref().map(|image| image.height),
            columns.byte_size as i64,
            columns.content_hash,
            request.source_app,
            request.captured_at,
        ],
    );
    if let Err(error) = inserted {
        discard_orphan_image(image_path.as_deref());
        return Err(database_failure(error));
    }
    Ok((transaction.last_insert_rowid(), image_path))
}

fn write_image(images_dir: &Path, png: &[u8]) -> Result<PathBuf, CommandError> {
    let file_name = format!("{}.png", Uuid::new_v4().simple());
    let final_path = images_dir.join(&file_name);
    let temporary_path = images_dir.join(format!("{file_name}.tmp"));
    fs::create_dir_all(images_dir)
        .and_then(|()| fs::write(&temporary_path, png))
        .and_then(|()| fs::rename(&temporary_path, &final_path))
        .map_err(|_| CommandError::Internal)?;
    Ok(final_path)
}

fn discard_orphan_image(path: Option<&Path>) {
    if let Some(path) = path {
        let _ = fs::remove_file(path);
    }
}

fn move_to_top(
    transaction: &Transaction,
    row_id: i64,
    columns: &Columns,
    request: &SaveRequest,
) -> Result<(), CommandError> {
    transaction
        .execute(
            "UPDATE items SET captured_at = ?1, source_app = ?2, html = COALESCE(?3, html) WHERE id = ?4",
            params![request.captured_at, request.source_app, columns.html, row_id],
        )
        .map_err(database_failure)?;
    Ok(())
}

fn database_failure(_: rusqlite::Error) -> CommandError {
    CommandError::Internal
}
