use super::{create, list};
use crate::db;
use crate::model::{CommandError, GroupId};
use rusqlite::Connection;

const CREATED_AT: i64 = 1_700_000_000_000;

fn migrated() -> Connection {
    let mut connection = Connection::open_in_memory().unwrap();
    db::migrate(&mut connection).unwrap();
    connection
}

#[test]
fn creating_a_group_in_an_empty_database_returns_it_at_the_first_position() {
    let connection = migrated();

    let group = create(&connection, "Work", CREATED_AT).unwrap();

    assert_eq!(group.id, GroupId(1));
    assert_eq!(group.name, "Work");
    assert_eq!(group.position, 0);
    assert!(!group.never_send_to_ai);
    assert_eq!(group.created_at, CREATED_AT);
}

#[test]
fn each_new_group_is_appended_after_the_existing_ones() {
    let connection = migrated();

    let first = create(&connection, "Work", CREATED_AT).unwrap();
    let second = create(&connection, "Home", CREATED_AT).unwrap();

    assert_eq!((first.id, first.position), (GroupId(1), 0));
    assert_eq!((second.id, second.position), (GroupId(2), 1));
}

#[test]
fn a_name_that_differs_only_by_case_is_a_conflict() {
    let connection = migrated();
    create(&connection, "Work", CREATED_AT).unwrap();

    let result = create(&connection, "WORK", CREATED_AT);

    assert_eq!(result, Err(CommandError::Conflict));
}

#[test]
fn a_blank_name_is_a_validation_error_on_the_name_field() {
    let connection = migrated();

    let result = create(&connection, "   ", CREATED_AT);

    assert_eq!(
        result,
        Err(CommandError::Validation {
            field: "name".to_owned()
        })
    );
}

#[test]
fn surrounding_whitespace_is_trimmed_from_the_name() {
    let connection = migrated();

    let group = create(&connection, "  Work\t", CREATED_AT).unwrap();

    assert_eq!(group.name, "Work");
}

#[test]
fn listing_an_empty_database_returns_no_groups() {
    let connection = migrated();

    assert_eq!(list(&connection), Ok(Vec::new()));
}

const NEWEST_LIVE_CAPTURED_AT: i64 = CREATED_AT + 5_000;
const DELETED_CAPTURED_AT: i64 = CREATED_AT + 9_000;

fn insert_live_item(connection: &Connection, group_id: GroupId, captured_at: i64) {
    connection
        .execute(
            "INSERT INTO items (group_id, kind, plain_text, byte_size, content_hash, captured_at) VALUES (?1, 'text', 'hello', 5, ?2, ?3)",
            rusqlite::params![group_id.0, captured_at.to_le_bytes(), captured_at],
        )
        .unwrap();
}

fn insert_deleted_item(connection: &Connection, group_id: GroupId, captured_at: i64) {
    connection
        .execute(
            "INSERT INTO items (group_id, kind, plain_text, byte_size, content_hash, captured_at, deleted_at) VALUES (?1, 'text', 'hello', 5, ?2, ?3, ?3)",
            rusqlite::params![group_id.0, captured_at.to_le_bytes(), captured_at],
        )
        .unwrap();
}

#[test]
fn listing_orders_groups_by_position_not_by_creation() {
    let connection = migrated();
    connection
        .execute_batch(
            "INSERT INTO groups (id, name, position, created_at) VALUES (1, 'Later', 1, 0), (2, 'Sooner', 0, 0)",
        )
        .unwrap();

    let names: Vec<_> = list(&connection)
        .unwrap()
        .into_iter()
        .map(|summary| summary.group.name)
        .collect();

    assert_eq!(names, ["Sooner", "Later"]);
}

#[test]
fn listing_counts_only_items_that_are_not_deleted() {
    let connection = migrated();
    let work = create(&connection, "Work", CREATED_AT).unwrap();
    insert_live_item(&connection, work.id, CREATED_AT + 1_000);
    insert_live_item(&connection, work.id, NEWEST_LIVE_CAPTURED_AT);
    insert_deleted_item(&connection, work.id, DELETED_CAPTURED_AT);

    let summaries = list(&connection).unwrap();

    assert_eq!(summaries[0].item_count, 2);
}

#[test]
fn listing_reports_the_newest_capture_among_items_that_are_not_deleted() {
    let connection = migrated();
    let work = create(&connection, "Work", CREATED_AT).unwrap();
    insert_live_item(&connection, work.id, CREATED_AT + 1_000);
    insert_live_item(&connection, work.id, NEWEST_LIVE_CAPTURED_AT);
    insert_deleted_item(&connection, work.id, DELETED_CAPTURED_AT);

    let summaries = list(&connection).unwrap();

    assert_eq!(summaries[0].last_activity, Some(NEWEST_LIVE_CAPTURED_AT));
}

#[test]
fn a_group_without_live_items_has_no_count_and_no_last_activity() {
    let connection = migrated();
    let empty = create(&connection, "Empty", CREATED_AT).unwrap();
    let cleared = create(&connection, "Cleared", CREATED_AT).unwrap();
    insert_deleted_item(&connection, cleared.id, DELETED_CAPTURED_AT);

    let summaries = list(&connection).unwrap();

    assert_eq!(empty.id, summaries[0].group.id);
    for summary in summaries {
        assert_eq!((summary.item_count, summary.last_activity), (0, None));
    }
}
