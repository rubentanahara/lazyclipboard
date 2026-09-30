use rusqlite::{params, Connection, Transaction};

const GROUP_COUNT: i64 = 20;
const ITEMS_PER_GROUP: i64 = 200;
const FIRST_CAPTURED_AT_MS: i64 = 1_700_000_000_000;
const CAPTURE_INTERVAL_MS: i64 = 1_000;
const PARAGRAPH: &str = "The quick brown fox jumps over the lazy dog. ";
const PARAGRAPH_REPEATS: usize = 44;
const IMAGE_WIDTH: i64 = 640;
const IMAGE_HEIGHT: i64 = 480;
const IMAGE_BYTE_SIZE: i64 = 48_000;
const SOURCE_APPS: [&str; 3] = ["Safari", "Code", "Terminal"];

pub fn load(connection: &mut Connection) -> rusqlite::Result<()> {
    let transaction = connection.transaction()?;
    for group in 0..GROUP_COUNT {
        insert_group(&transaction, group)?;
        for index in 0..ITEMS_PER_GROUP {
            insert_item(&transaction, group, index)?;
        }
    }
    transaction.commit()
}

fn insert_group(transaction: &Transaction, group: i64) -> rusqlite::Result<()> {
    transaction
        .prepare_cached(
            "INSERT INTO groups (id, name, position, never_send_to_ai, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        )?
        .execute(params![
            group + 1,
            format!("Group {group:02}"),
            group,
            i64::from(group % 10 == 9),
            FIRST_CAPTURED_AT_MS,
        ])?;
    Ok(())
}

fn insert_item(transaction: &Transaction, group: i64, index: i64) -> rusqlite::Result<()> {
    let body = format!(
        "Group {group} item {index}. {}",
        PARAGRAPH.repeat(PARAGRAPH_REPEATS)
    );
    let (kind, plain_text, html, image_file, byte_size) = match index % 10 {
        6 | 7 => (
            "rich_text",
            Some(body.clone()),
            Some(format!("<p>{body}</p>")),
            None,
            body.len() as i64,
        ),
        8 => {
            let link = format!("https://example.com/group-{group}/item-{index}");
            let size = link.len() as i64;
            ("link", Some(link), None, None, size)
        }
        9 => (
            "image",
            None,
            None,
            Some(format!("seed-{group}-{index}.png")),
            IMAGE_BYTE_SIZE,
        ),
        _ => ("text", Some(body.clone()), None, None, body.len() as i64),
    };
    let is_image = kind == "image";
    transaction
        .prepare_cached(
            "INSERT INTO items (group_id, kind, plain_text, html, image_file, image_width, image_height, byte_size, content_hash, source_app, captured_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        )?
        .execute(params![
            group + 1,
            kind,
            plain_text,
            html,
            image_file,
            is_image.then_some(IMAGE_WIDTH),
            is_image.then_some(IMAGE_HEIGHT),
            byte_size,
            content_hash(group, index),
            SOURCE_APPS.get(index as usize % 4).copied(),
            FIRST_CAPTURED_AT_MS + (group * ITEMS_PER_GROUP + index) * CAPTURE_INTERVAL_MS,
        ])?;
    Ok(())
}

fn content_hash(group: i64, index: i64) -> [u8; 32] {
    let mut hash = [0u8; 32];
    hash[..8].copy_from_slice(&group.to_le_bytes());
    hash[8..16].copy_from_slice(&index.to_le_bytes());
    hash
}
