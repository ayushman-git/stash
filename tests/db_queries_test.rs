// Database query unit tests
mod common;

use stash::db::queries;
use common::{setup_test_db, create_new_article};

// CRUD Operations Tests

#[test]
fn test_insert_article_success() {
    let conn = setup_test_db();
    let new_article = create_new_article(
        "abc12345",
        "https://example.com/article",
        Some("Test Article"),
        vec!["rust", "testing"],
    );

    let result = queries::insert_article(&conn, new_article);
    assert!(result.is_ok());
    
    let article = result.unwrap();
    assert_eq!(article.id, 1);
    assert_eq!(article.hash, "abc12345");
    assert_eq!(article.title, Some("Test Article".to_string()));
    assert_eq!(article.tags, vec!["rust", "testing"]);
    assert!(!article.read);
    assert!(!article.archived);
}

#[test]
fn test_insert_article_duplicate_hash_fails() {
    let conn = setup_test_db();
    let article1 = create_new_article("samehash", "https://example.com/1", Some("First"), vec![]);
    let article2 = create_new_article("samehash", "https://example.com/2", Some("Second"), vec![]);

    queries::insert_article(&conn, article1).expect("First insert should succeed");
    let result = queries::insert_article(&conn, article2);
    
    assert!(result.is_err());
}

#[test]
fn test_find_by_hash_existing() {
    let conn = setup_test_db();
    let new_article = create_new_article("findhash", "https://example.com", Some("Find Me"), vec![]);
    queries::insert_article(&conn, new_article).unwrap();

    let result = queries::find_by_hash(&conn, "findhash");
    assert!(result.is_ok());
    
    let found = result.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().hash, "findhash");
}

#[test]
fn test_find_by_hash_nonexistent() {
    let conn = setup_test_db();
    
    let result = queries::find_by_hash(&conn, "doesnotexist");
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_find_by_ids_multiple() {
    let conn = setup_test_db();
    
    // Insert 3 articles
    for i in 1..=3 {
        let article = create_new_article(
            &format!("hash{}", i),
            &format!("https://example.com/{}", i),
            Some(&format!("Article {}", i)),
            vec![],
        );
        queries::insert_article(&conn, article).unwrap();
    }

    let result = queries::find_by_ids(&conn, &[1, 3]);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 2);
    assert_eq!(articles[0].id, 1);
    assert_eq!(articles[1].id, 3);
}

#[test]
fn test_find_by_ids_empty_array() {
    let conn = setup_test_db();
    
    let result = queries::find_by_ids(&conn, &[]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_get_article_by_id_existing() {
    let conn = setup_test_db();
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    queries::insert_article(&conn, article).unwrap();

    let result = queries::get_article_by_id(&conn, 1);
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());
}

#[test]
fn test_get_article_by_id_nonexistent() {
    let conn = setup_test_db();
    
    let result = queries::get_article_by_id(&conn, 999);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

// List Operations Tests

#[test]
fn test_list_articles_default_unread_only() {
    let conn = setup_test_db();
    
    // Insert 2 unread and 1 read article
    for i in 1..=3 {
        let article = create_new_article(
            &format!("hash{}", i),
            &format!("https://example.com/{}", i),
            Some(&format!("Article {}", i)),
            vec![],
        );
        let inserted = queries::insert_article(&conn, article).unwrap();
        
        if i == 3 {
            queries::mark_read_by_ids(&conn, &[inserted.id]).unwrap();
        }
    }

    let result = queries::list_articles(&conn, 10, false);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 2); // Only unread articles
}

#[test]
fn test_list_articles_with_all_flag() {
    let conn = setup_test_db();
    
    // Insert 3 articles with mixed states
    for i in 1..=3 {
        let article = create_new_article(
            &format!("hash{}", i),
            &format!("https://example.com/{}", i),
            Some(&format!("Article {}", i)),
            vec![],
        );
        queries::insert_article(&conn, article).unwrap();
    }

    let result = queries::list_articles(&conn, 10, true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn test_list_articles_respects_limit() {
    let conn = setup_test_db();
    
    // Insert 5 articles
    for i in 1..=5 {
        let article = create_new_article(
            &format!("hash{}", i),
            &format!("https://example.com/{}", i),
            Some(&format!("Article {}", i)),
            vec![],
        );
        queries::insert_article(&conn, article).unwrap();
    }

    let result = queries::list_articles(&conn, 3, true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn test_list_articles_filtered_by_single_tag() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("Rust"), vec!["rust"]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("Python"), vec!["python"]);
    let article3 = create_new_article("hash3", "https://example.com/3", Some("Rust2"), vec!["rust", "cli"]);
    
    queries::insert_article(&conn, article1).unwrap();
    queries::insert_article(&conn, article2).unwrap();
    queries::insert_article(&conn, article3).unwrap();

    let result = queries::list_articles_filtered(&conn, 10, true, false, false, &["rust".to_string()], "time", false);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 2);
}

#[test]
fn test_list_articles_filtered_by_multiple_tags_and_logic() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("Both"), vec!["rust", "cli"]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("One"), vec!["rust"]);
    let article3 = create_new_article("hash3", "https://example.com/3", Some("Other"), vec!["cli"]);
    
    queries::insert_article(&conn, article1).unwrap();
    queries::insert_article(&conn, article2).unwrap();
    queries::insert_article(&conn, article3).unwrap();

    let result = queries::list_articles_filtered(
        &conn, 10, true, false, false,
        &["rust".to_string(), "cli".to_string()],
        "time", false
    );
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 1); // Only article with BOTH tags
    assert_eq!(articles[0].title, Some("Both".to_string()));
}

#[test]
fn test_list_articles_filtered_by_starred() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("Normal"), vec![]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("Star"), vec![]);
    
    let _id1 = queries::insert_article(&conn, article1).unwrap().id;
    let id2 = queries::insert_article(&conn, article2).unwrap().id;
    
    queries::set_starred_by_ids(&conn, &[id2], true).unwrap();

    let result = queries::list_articles_filtered(&conn, 10, true, false, true, &[], "time", false);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].id, id2);
}

#[test]
fn test_list_articles_filtered_by_archived() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("Active"), vec![]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("Archived"), vec![]);
    
    let _id1 = queries::insert_article(&conn, article1).unwrap().id;
    let id2 = queries::insert_article(&conn, article2).unwrap().id;
    
    queries::archive_by_ids(&conn, &[id2]).unwrap();

    let result = queries::list_articles_filtered(&conn, 10, true, true, false, &[], "time", false);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 1);
    assert_eq!(articles[0].id, id2);
}

#[test]
fn test_list_articles_sorted_by_title() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("Zebra"), vec![]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("Apple"), vec![]);
    let article3 = create_new_article("hash3", "https://example.com/3", Some("Banana"), vec![]);
    
    queries::insert_article(&conn, article1).unwrap();
    queries::insert_article(&conn, article2).unwrap();
    queries::insert_article(&conn, article3).unwrap();

    let result = queries::list_articles_filtered(&conn, 10, true, false, false, &[], "title", false);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles[0].title, Some("Apple".to_string()));
    assert_eq!(articles[1].title, Some("Banana".to_string()));
    assert_eq!(articles[2].title, Some("Zebra".to_string()));
}

#[test]
fn test_list_articles_sorted_by_title_reverse() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("Apple"), vec![]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("Zebra"), vec![]);
    
    queries::insert_article(&conn, article1).unwrap();
    queries::insert_article(&conn, article2).unwrap();

    let result = queries::list_articles_filtered(&conn, 10, true, false, false, &[], "title", true);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles[0].title, Some("Zebra".to_string()));
    assert_eq!(articles[1].title, Some("Apple".to_string()));
}

#[test]
fn test_get_random_articles() {
    let conn = setup_test_db();
    
    // Insert 5 articles
    for i in 1..=5 {
        let article = create_new_article(
            &format!("hash{}", i),
            &format!("https://example.com/{}", i),
            Some(&format!("Article {}", i)),
            vec![],
        );
        queries::insert_article(&conn, article).unwrap();
    }

    let result = queries::get_random_articles(&conn, 3, true);
    assert!(result.is_ok());
    
    let articles = result.unwrap();
    assert_eq!(articles.len(), 3);
}

// State Management Tests

#[test]
fn test_mark_read_by_ids() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;

    let result = queries::mark_read_by_ids(&conn, &[id]);
    assert!(result.is_ok());
    
    let updated = result.unwrap();
    assert_eq!(updated.len(), 1);
    assert!(updated[0].read);
}

#[test]
fn test_set_read_by_ids_unread() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;
    
    queries::mark_read_by_ids(&conn, &[id]).unwrap();
    
    let result = queries::set_read_by_ids(&conn, &[id], false);
    assert!(result.is_ok());
    
    let updated = result.unwrap();
    assert!(!updated[0].read);
}

#[test]
fn test_set_read_all() {
    let conn = setup_test_db();
    
    // Insert 3 articles
    for i in 1..=3 {
        let article = create_new_article(
            &format!("hash{}", i),
            &format!("https://example.com/{}", i),
            Some(&format!("Article {}", i)),
            vec![],
        );
        queries::insert_article(&conn, article).unwrap();
    }

    let result = queries::set_read_all(&conn, true, false);
    assert!(result.is_ok());
    
    let updated = result.unwrap();
    assert_eq!(updated.len(), 3);
    assert!(updated.iter().all(|a| a.read));
}

#[test]
fn test_set_starred_by_ids() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;

    let result = queries::set_starred_by_ids(&conn, &[id], true);
    assert!(result.is_ok());
    
    let updated = result.unwrap();
    assert_eq!(updated.len(), 1);
    assert!(updated[0].starred);
}

#[test]
fn test_archive_by_ids() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;

    let result = queries::archive_by_ids(&conn, &[id]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
    
    let archived = queries::get_article_by_id(&conn, id).unwrap().unwrap();
    assert!(archived.archived);
}

#[test]
fn test_unarchive_by_ids() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;
    
    queries::archive_by_ids(&conn, &[id]).unwrap();
    
    let result = queries::unarchive_by_ids(&conn, &[id]);
    assert!(result.is_ok());
    
    let unarchived = queries::get_article_by_id(&conn, id).unwrap().unwrap();
    assert!(!unarchived.archived);
    assert!(!unarchived.read); // Should also reset read status
}

#[test]
fn test_delete_by_ids() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;

    let result = queries::delete_by_ids(&conn, &[id]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
    
    let deleted = queries::get_article_by_id(&conn, id).unwrap();
    assert!(deleted.is_none());
}

#[test]
fn test_delete_by_ids_empty_array() {
    let conn = setup_test_db();
    
    let result = queries::delete_by_ids(&conn, &[]);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

// Tag Operations Tests

#[test]
fn test_update_tags() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Test"), vec!["old"]);
    let id = queries::insert_article(&conn, article).unwrap().id;

    let new_tags = vec!["rust".to_string(), "cli".to_string()];
    let result = queries::update_tags(&conn, id, new_tags.clone());
    if let Err(e) = &result {
        eprintln!("Error updating tags: {:?}", e);
    }
    assert!(result.is_ok());
    
    let updated = result.unwrap();
    assert_eq!(updated.tags, new_tags);
}

#[test]
fn test_get_all_tags_with_counts() {
    let conn = setup_test_db();
    
    let article1 = create_new_article("hash1", "https://example.com/1", Some("A"), vec!["rust", "cli"]);
    let article2 = create_new_article("hash2", "https://example.com/2", Some("B"), vec!["rust", "web"]);
    let article3 = create_new_article("hash3", "https://example.com/3", Some("C"), vec!["python"]);
    
    queries::insert_article(&conn, article1).unwrap();
    queries::insert_article(&conn, article2).unwrap();
    queries::insert_article(&conn, article3).unwrap();

    let result = queries::get_all_tags_with_counts(&conn);
    assert!(result.is_ok());
    
    let tags = result.unwrap();
    assert_eq!(tags.len(), 4);
    
    // Find rust tag
    let rust_tag = tags.iter().find(|(name, _)| name == "rust");
    assert!(rust_tag.is_some());
    assert_eq!(rust_tag.unwrap().1, 2);
}

#[test]
fn test_update_article_metadata() {
    let conn = setup_test_db();
    
    let article = create_new_article("hash1", "https://example.com", Some("Old Title"), vec![]);
    let id = queries::insert_article(&conn, article).unwrap().id;

    let result = queries::update_article_metadata(
        &conn,
        id,
        Some("New Title".to_string()),
        "https://example.com/new".to_string(),
        Some("A note".to_string()),
        vec!["updated".to_string()],
        true,
        true,
        false,
    );
    
    if let Err(e) = &result {
        eprintln!("Error updating metadata: {:?}", e);
    }
    assert!(result.is_ok());
    
    let updated = result.unwrap();
    assert_eq!(updated.title, Some("New Title".to_string()));
    assert_eq!(updated.url, "https://example.com/new");
    assert_eq!(updated.note, Some("A note".to_string()));
    assert!(updated.starred);
    assert!(updated.read);
    assert!(!updated.archived);
}

