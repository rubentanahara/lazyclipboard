use super::{classify, save, Classified, Flavours, PendingItem, RgbaImage, SaveRequest};
use crate::db;
use crate::model::{CommandError, GroupId, ItemId};
use rusqlite::Connection;

fn plain(text: &str) -> Flavours {
    Flavours {
        plain_text: Some(text.to_owned()),
        ..Flavours::default()
    }
}

fn stored(item: PendingItem) -> Classified {
    Classified {
        item: Some(item),
        skipped_images: 0,
    }
}

#[test]
fn plain_text_is_classified_as_text() {
    let classified = classify(plain("hello world")).unwrap();

    assert_eq!(
        classified,
        stored(PendingItem::Text {
            text: "hello world".to_owned()
        })
    );
}

#[test]
fn a_bare_url_is_classified_as_a_link_without_surrounding_whitespace() {
    let classified = classify(plain("  https://example.com/a?b=1#c\n")).unwrap();

    assert_eq!(
        classified,
        stored(PendingItem::Link {
            url: "https://example.com/a?b=1#c".to_owned()
        })
    );
}

#[test]
fn text_that_only_contains_a_url_stays_text() {
    for text in [
        "see https://example.com",
        "https://example.com and more",
        "https://",
        "ftp://example.com",
        "HTTPS://example.com",
    ] {
        let classified = classify(plain(text)).unwrap();

        assert_eq!(
            classified,
            stored(PendingItem::Text {
                text: text.to_owned()
            }),
            "{text:?}"
        );
    }
}

const FORMATTED_BROWSER_COPY: &str = include_str!("fixtures/browser_bold_and_link.html");
const PLAIN_BROWSER_COPY: &str = include_str!("fixtures/browser_plain_span.html");
const SCRIPT_BROWSER_COPY: &str = include_str!("fixtures/browser_script_and_handlers.html");

fn with_html(plain_text: &str, html: &str) -> Flavours {
    Flavours {
        plain_text: Some(plain_text.to_owned()),
        html: Some(html.to_owned()),
        ..Flavours::default()
    }
}

#[test]
fn html_with_real_formatting_is_rich_text_with_sanitised_html_and_plain_fallback() {
    let classified = classify(with_html(
        "Release notes: read the changelog",
        FORMATTED_BROWSER_COPY,
    ))
    .unwrap();

    assert_eq!(
        classified,
        stored(PendingItem::RichText {
            plain_text: "Release notes: read the changelog".to_owned(),
            html: "<strong>Release notes:</strong><span> read the </span><a href=\"https://example.com/changelog\" rel=\"noopener noreferrer\">changelog</a>".to_owned(),
        })
    );
}

#[test]
fn html_with_only_structural_tags_is_plain_text() {
    let classified = classify(with_html("just some selected words", PLAIN_BROWSER_COPY)).unwrap();

    assert_eq!(
        classified,
        stored(PendingItem::Text {
            text: "just some selected words".to_owned()
        })
    );
}

#[test]
fn scripts_handlers_and_unsafe_urls_never_reach_the_stored_html() {
    let classified = classify(with_html("bold trap", SCRIPT_BROWSER_COPY)).unwrap();

    assert_eq!(
        classified,
        stored(PendingItem::RichText {
            plain_text: "bold trap".to_owned(),
            html: "<b>bold</b><a rel=\"noopener noreferrer\">trap</a>".to_owned(),
        })
    );
}

const ONE_MEGABYTE: usize = 1024 * 1024;

#[test]
fn plain_text_over_one_megabyte_is_too_large_and_exactly_one_megabyte_is_kept() {
    let at_the_cap = classify(plain(&"a".repeat(ONE_MEGABYTE)));
    let over_the_cap = classify(plain(&"a".repeat(ONE_MEGABYTE + 1)));

    assert!(matches!(
        at_the_cap,
        Ok(Classified {
            item: Some(PendingItem::Text { .. }),
            ..
        })
    ));
    assert!(matches!(over_the_cap, Err(CommandError::TooLarge)));
}

#[test]
fn html_over_two_megabytes_is_dropped_and_the_plain_text_is_kept() {
    let formatted = |bytes: usize| format!("<b>{}</b>", "a".repeat(bytes - "<b></b>".len()));
    let at_the_cap = classify(with_html("a", &formatted(2 * ONE_MEGABYTE)));
    let over_the_cap = classify(with_html("a", &formatted(2 * ONE_MEGABYTE + 1)));

    assert!(matches!(
        at_the_cap,
        Ok(Classified {
            item: Some(PendingItem::RichText { .. }),
            ..
        })
    ));
    assert!(matches!(
        over_the_cap,
        Ok(Classified {
            item: Some(PendingItem::Text { ref text }),
            ..
        }) if text == "a"
    ));
}

#[test]
fn content_without_usable_plain_text_or_image_is_unsupported() {
    for flavours in [
        Flavours::default(),
        plain(""),
        plain(" \n\t "),
        Flavours {
            html: Some(FORMATTED_BROWSER_COPY.to_owned()),
            ..Flavours::default()
        },
    ] {
        assert_eq!(classify(flavours), Err(CommandError::Unsupported));
    }
}

const TWO_BY_TWO_PIXELS: [u8; 16] = [
    255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 0,
];

fn two_by_two_image() -> RgbaImage {
    RgbaImage {
        width: 2,
        height: 2,
        rgba: TWO_BY_TWO_PIXELS.to_vec(),
    }
}

fn decode_png(png: &[u8]) -> (png::OutputInfo, Vec<u8>) {
    let mut reader = png::Decoder::new(std::io::Cursor::new(png))
        .read_info()
        .unwrap();
    let mut pixels = vec![0; reader.output_buffer_size().unwrap()];
    let info = reader.next_frame(&mut pixels).unwrap();
    (info, pixels)
}

#[test]
fn an_image_is_re_encoded_to_a_png_that_decodes_to_the_same_pixels() {
    let classified = classify(Flavours {
        image: Some(two_by_two_image()),
        ..Flavours::default()
    })
    .unwrap();

    let Some(PendingItem::Image { png, width, height }) = classified.item else {
        panic!("expected an image item, got {classified:?}");
    };
    let (info, pixels) = decode_png(&png);
    assert_eq!((width, height), (2, 2));
    assert_eq!((info.width, info.height), (2, 2));
    assert_eq!(info.color_type, png::ColorType::Rgba);
    assert_eq!(pixels, TWO_BY_TWO_PIXELS);
}

#[test]
fn an_image_wins_over_text_and_formatted_html() {
    let classified = classify(Flavours {
        plain_text: Some("Release notes: read the changelog".to_owned()),
        html: Some(FORMATTED_BROWSER_COPY.to_owned()),
        image: Some(two_by_two_image()),
    })
    .unwrap();

    assert!(matches!(
        classified.item,
        Some(PendingItem::Image {
            width: 2,
            height: 2,
            ..
        })
    ));
}

const INCOMPRESSIBLE_SIDE: u32 = 1800;

fn incompressible_image() -> RgbaImage {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    let rgba = (0..INCOMPRESSIBLE_SIDE * INCOMPRESSIBLE_SIDE * 4)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state >> 24) as u8
        })
        .collect();
    RgbaImage {
        width: INCOMPRESSIBLE_SIDE,
        height: INCOMPRESSIBLE_SIDE,
        rgba,
    }
}

#[test]
fn an_image_over_ten_megabytes_is_skipped_and_counted_and_the_text_flavour_is_kept() {
    let classified = classify(Flavours {
        plain_text: Some("caption".to_owned()),
        image: Some(incompressible_image()),
        ..Flavours::default()
    })
    .unwrap();

    assert_eq!(
        classified,
        Classified {
            item: Some(PendingItem::Text {
                text: "caption".to_owned()
            }),
            skipped_images: 1,
        }
    );
}

#[test]
fn an_oversized_image_alone_stores_nothing_and_reports_one_skipped() {
    let classified = classify(Flavours {
        image: Some(incompressible_image()),
        ..Flavours::default()
    })
    .unwrap();

    assert_eq!(
        classified,
        Classified {
            item: None,
            skipped_images: 1,
        }
    );
}

#[test]
fn pixel_data_that_does_not_match_the_dimensions_is_a_validation_error() {
    for (width, height, byte_count) in [(2, 2, 15), (2, 2, 17), (0, 0, 0), (0, 2, 8)] {
        let result = classify(Flavours {
            image: Some(RgbaImage {
                width,
                height,
                rgba: vec![0; byte_count],
            }),
            ..Flavours::default()
        });

        assert_eq!(
            result,
            Err(CommandError::Validation {
                field: "image".to_owned()
            }),
            "{width}x{height} with {byte_count} bytes"
        );
    }
}

const WORK_GROUP: GroupId = GroupId(1);
const NOTES_GROUP: GroupId = GroupId(2);

struct Store {
    connection: Connection,
    images: tempfile::TempDir,
}

fn store_with_groups() -> Store {
    let mut connection = Connection::open_in_memory().unwrap();
    db::migrate(&mut connection).unwrap();
    for (id, name) in [(WORK_GROUP, "Work"), (NOTES_GROUP, "Notes")] {
        connection
            .execute(
                "INSERT INTO groups (id, name, position, created_at) VALUES (?1, ?2, ?1, 0)",
                rusqlite::params![id.0, name],
            )
            .unwrap();
    }
    Store {
        connection,
        images: tempfile::tempdir().unwrap(),
    }
}

fn text_request(group: GroupId, text: &str, captured_at: i64) -> SaveRequest {
    SaveRequest {
        group,
        item: PendingItem::Text {
            text: text.to_owned(),
        },
        source_app: None,
        captured_at,
    }
}

#[derive(Debug, PartialEq)]
struct Row {
    group_id: u32,
    kind: String,
    plain_text: Option<String>,
    html: Option<String>,
    source_app: Option<String>,
    byte_size: i64,
    captured_at: i64,
}

fn row(connection: &Connection, id: ItemId) -> Row {
    connection
        .query_row(
            "SELECT group_id, kind, plain_text, html, source_app, byte_size, captured_at FROM items WHERE id = ?1",
            [id.0],
            |row| {
                Ok(Row {
                    group_id: row.get(0)?,
                    kind: row.get(1)?,
                    plain_text: row.get(2)?,
                    html: row.get(3)?,
                    source_app: row.get(4)?,
                    byte_size: row.get(5)?,
                    captured_at: row.get(6)?,
                })
            },
        )
        .unwrap()
}

#[test]
fn saving_text_stores_a_text_row_in_the_chosen_group() {
    let mut store = store_with_groups();
    let request = SaveRequest {
        source_app: Some("Safari".to_owned()),
        ..text_request(WORK_GROUP, "hello", 1_700_000_000_000)
    };

    let id = save(&mut store.connection, store.images.path(), request).unwrap();

    assert_eq!(
        row(&store.connection, id),
        Row {
            group_id: 1,
            kind: "text".to_owned(),
            plain_text: Some("hello".to_owned()),
            html: None,
            source_app: Some("Safari".to_owned()),
            byte_size: 5,
            captured_at: 1_700_000_000_000,
        }
    );
}

fn item_ids_newest_first(connection: &Connection, group: GroupId) -> Vec<ItemId> {
    connection
        .prepare("SELECT id FROM items WHERE group_id = ?1 AND deleted_at IS NULL ORDER BY captured_at DESC")
        .unwrap()
        .query_map([group.0], |row| row.get(0).map(ItemId))
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap()
}

#[test]
fn capturing_an_identical_item_again_moves_it_to_the_top_without_adding_a_row() {
    let mut store = store_with_groups();
    let images = store.images.path();
    let first = save(
        &mut store.connection,
        images,
        text_request(WORK_GROUP, "alpha", 1_000),
    )
    .unwrap();
    let second = save(
        &mut store.connection,
        images,
        text_request(WORK_GROUP, "beta", 2_000),
    )
    .unwrap();
    assert_eq!(
        item_ids_newest_first(&store.connection, WORK_GROUP),
        [second, first]
    );

    let again = save(
        &mut store.connection,
        images,
        SaveRequest {
            source_app: Some("Terminal".to_owned()),
            ..text_request(WORK_GROUP, "alpha", 3_000)
        },
    )
    .unwrap();

    assert_eq!(again, first);
    assert_eq!(
        item_ids_newest_first(&store.connection, WORK_GROUP),
        [first, second]
    );
    let moved = row(&store.connection, first);
    assert_eq!(moved.captured_at, 3_000);
    assert_eq!(moved.source_app.as_deref(), Some("Terminal"));
}

fn request_for(group: GroupId, item: PendingItem, captured_at: i64) -> SaveRequest {
    SaveRequest {
        group,
        item,
        source_app: None,
        captured_at,
    }
}

fn rich_text(plain_text: &str, html: &str) -> PendingItem {
    PendingItem::RichText {
        plain_text: plain_text.to_owned(),
        html: html.to_owned(),
    }
}

#[test]
fn saving_rich_text_keeps_the_sanitised_html_and_the_plain_fallback() {
    let mut store = store_with_groups();

    let id = save(
        &mut store.connection,
        store.images.path(),
        request_for(WORK_GROUP, rich_text("bold", "<b>bold</b>"), 10),
    )
    .unwrap();

    assert_eq!(
        row(&store.connection, id),
        Row {
            group_id: 1,
            kind: "rich_text".to_owned(),
            plain_text: Some("bold".to_owned()),
            html: Some("<b>bold</b>".to_owned()),
            source_app: None,
            byte_size: 15,
            captured_at: 10,
        }
    );
}

#[test]
fn saving_a_link_stores_a_link_row_with_the_url_as_plain_text() {
    let mut store = store_with_groups();
    let link = PendingItem::Link {
        url: "https://example.com".to_owned(),
    };

    let id = save(
        &mut store.connection,
        store.images.path(),
        request_for(WORK_GROUP, link, 10),
    )
    .unwrap();

    let stored = row(&store.connection, id);
    assert_eq!(stored.kind, "link");
    assert_eq!(stored.plain_text.as_deref(), Some("https://example.com"));
    assert_eq!(stored.html, None);
}

#[test]
fn recapturing_rich_text_with_the_same_plain_text_refreshes_its_html() {
    let mut store = store_with_groups();
    let images = store.images.path();
    let first = save(
        &mut store.connection,
        images,
        request_for(WORK_GROUP, rich_text("note", "<i>note</i>"), 10),
    )
    .unwrap();

    let again = save(
        &mut store.connection,
        images,
        request_for(WORK_GROUP, rich_text("note", "<b>note</b>"), 20),
    )
    .unwrap();

    assert_eq!(again, first);
    assert_eq!(
        row(&store.connection, first).html.as_deref(),
        Some("<b>note</b>")
    );
}

#[test]
fn the_same_string_as_text_and_as_a_link_are_different_items() {
    let mut store = store_with_groups();
    let images = store.images.path();
    let as_text = request_for(
        WORK_GROUP,
        PendingItem::Text {
            text: "https://example.com".to_owned(),
        },
        10,
    );
    let as_link = request_for(
        WORK_GROUP,
        PendingItem::Link {
            url: "https://example.com".to_owned(),
        },
        20,
    );

    let text_id = save(&mut store.connection, images, as_text).unwrap();
    let link_id = save(&mut store.connection, images, as_link).unwrap();

    assert_ne!(text_id, link_id);
}

fn pending_image() -> (PendingItem, Vec<u8>) {
    let classified = classify(Flavours {
        image: Some(two_by_two_image()),
        ..Flavours::default()
    })
    .unwrap();
    let item = classified.item.unwrap();
    let PendingItem::Image { png, .. } = &item else {
        panic!("expected an image item, got {item:?}");
    };
    let png = png.clone();
    (item, png)
}

fn file_names(directory: &std::path::Path) -> Vec<String> {
    std::fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}

#[test]
fn saving_an_image_writes_a_png_file_and_stores_its_name_and_size() {
    let mut store = store_with_groups();
    let (item, png) = pending_image();

    let id = save(
        &mut store.connection,
        store.images.path(),
        request_for(WORK_GROUP, item, 10),
    )
    .unwrap();

    let (kind, plain_text, image_file, width, height, byte_size): (String, Option<String>, String, u32, u32, i64) = store
        .connection
        .query_row(
            "SELECT kind, plain_text, image_file, image_width, image_height, byte_size FROM items WHERE id = ?1",
            [id.0],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
        )
        .unwrap();
    assert_eq!(
        (kind.as_str(), plain_text, width, height, byte_size),
        ("image", None, 2, 2, png.len() as i64)
    );
    assert!(image_file.ends_with(".png"), "{image_file}");
    assert_eq!(
        file_names(store.images.path()),
        std::slice::from_ref(&image_file)
    );
    assert_eq!(
        std::fs::read(store.images.path().join(image_file)).unwrap(),
        png
    );
}

#[test]
fn recapturing_an_identical_image_keeps_one_row_and_one_file() {
    let mut store = store_with_groups();
    let images = store.images.path();
    let (item, _) = pending_image();

    let first = save(
        &mut store.connection,
        images,
        request_for(WORK_GROUP, item.clone(), 10),
    )
    .unwrap();
    let again = save(
        &mut store.connection,
        images,
        request_for(WORK_GROUP, item, 20),
    )
    .unwrap();

    assert_eq!(again, first);
    assert_eq!(file_names(images).len(), 1);
    assert_eq!(row_count(&store.connection), 1);
}

fn row_count(connection: &Connection) -> i64 {
    connection
        .query_row("SELECT count(*) FROM items", [], |row| row.get(0))
        .unwrap()
}

#[test]
fn saving_into_a_group_that_does_not_exist_is_not_found_and_leaves_no_file() {
    let mut store = store_with_groups();
    let (item, _) = pending_image();

    let result = save(
        &mut store.connection,
        store.images.path(),
        request_for(GroupId(99), item, 10),
    );

    assert_eq!(result, Err(CommandError::NotFound));
    assert_eq!(row_count(&store.connection), 0);
    assert!(file_names(store.images.path()).is_empty());
}

#[test]
fn the_same_item_in_two_groups_is_two_rows() {
    let mut store = store_with_groups();
    let images = store.images.path();

    let in_work = save(
        &mut store.connection,
        images,
        text_request(WORK_GROUP, "shared", 10),
    )
    .unwrap();
    let in_notes = save(
        &mut store.connection,
        images,
        text_request(NOTES_GROUP, "shared", 20),
    )
    .unwrap();

    assert_ne!(in_work, in_notes);
    assert_eq!(row_count(&store.connection), 2);
}

#[test]
fn a_soft_deleted_item_is_not_revived_by_capturing_it_again() {
    let mut store = store_with_groups();
    let images = store.images.path();
    let deleted = save(
        &mut store.connection,
        images,
        text_request(WORK_GROUP, "gone", 10),
    )
    .unwrap();
    store
        .connection
        .execute(
            "UPDATE items SET deleted_at = 15 WHERE id = ?1",
            [deleted.0],
        )
        .unwrap();

    let captured_again = save(
        &mut store.connection,
        images,
        text_request(WORK_GROUP, "gone", 20),
    )
    .unwrap();

    assert_ne!(captured_again, deleted);
    assert_eq!(
        item_ids_newest_first(&store.connection, WORK_GROUP),
        [captured_again]
    );
}
