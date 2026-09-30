use rusqlite::Connection;

fn table_names(connection: &Connection) -> Vec<String> {
    let mut statement = connection
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name")
        .unwrap();
    statement
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

#[test]
fn migrating_an_empty_database_creates_schema_v1() {
    let mut connection = Connection::open_in_memory().unwrap();

    super::migrate(&mut connection).unwrap();

    assert_eq!(
        table_names(&connection),
        ["groups", "items", "settings", "usage_daily"]
    );
    let version: i64 = connection
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap();
    assert_eq!(version, 1);
}

fn count(connection: &Connection, sql: &str) -> i64 {
    connection.query_row(sql, [], |row| row.get(0)).unwrap()
}

#[test]
fn seed_loads_twenty_groups_of_two_hundred_items_in_under_a_second() {
    let started = std::time::Instant::now();
    let mut connection = Connection::open_in_memory().unwrap();

    super::migrate(&mut connection).unwrap();
    super::seed::load(&mut connection).unwrap();

    assert!(started.elapsed() < std::time::Duration::from_secs(1));
    assert_eq!(count(&connection, "SELECT count(*) FROM groups"), 20);
    assert_eq!(
        count(
            &connection,
            "SELECT count(*) FROM (SELECT group_id FROM items GROUP BY group_id HAVING count(*) = 200)"
        ),
        20
    );
    assert_eq!(count(&connection, "SELECT count(*) FROM items"), 4000);
    for kind in ["text", "rich_text", "link", "image"] {
        let of_kind = count(
            &connection,
            &format!("SELECT count(*) FROM items WHERE kind = '{kind}'"),
        );
        assert!(of_kind > 0, "no {kind} items in the seed");
    }
}

fn migrated_with_two_groups() -> Connection {
    let mut connection = Connection::open_in_memory().unwrap();
    super::migrate(&mut connection).unwrap();
    connection
        .execute_batch(
            "INSERT INTO groups (id, name, position, created_at) VALUES (1, 'Work', 0, 0), (2, 'Home', 1, 0)",
        )
        .unwrap();
    connection
}

fn insert_text(connection: &Connection, group_id: i64, hash: u8) -> rusqlite::Result<usize> {
    connection.execute(
        "INSERT INTO items (group_id, kind, plain_text, byte_size, content_hash, captured_at) VALUES (?1, 'text', 'hello', 5, ?2, 0)",
        rusqlite::params![group_id, [hash]],
    )
}

#[test]
fn identical_content_is_rejected_within_a_group_but_allowed_in_another() {
    let connection = migrated_with_two_groups();

    insert_text(&connection, 1, 1).unwrap();

    assert!(insert_text(&connection, 1, 1).is_err());
    assert!(insert_text(&connection, 2, 1).is_ok());
}

#[test]
fn an_image_item_without_an_image_file_is_rejected() {
    let connection = migrated_with_two_groups();

    let result = connection.execute(
        "INSERT INTO items (group_id, kind, byte_size, content_hash, captured_at) VALUES (1, 'image', 10, x'01', 0)",
        [],
    );

    assert!(result.is_err());
}

#[test]
fn group_names_are_unique_ignoring_case() {
    let connection = migrated_with_two_groups();

    let result = connection.execute(
        "INSERT INTO groups (name, position, created_at) VALUES ('WORK', 2, 0)",
        [],
    );

    assert!(result.is_err());
}
