// URL processing and hash generation tests
use blake3;

// Hash Generation Tests

#[test]
fn test_blake3_hash_consistency() {
    let url = "https://example.com/article";
    
    let hash1: String = blake3::hash(url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_eq!(hash1, hash2);
}

#[test]
fn test_blake3_hash_8_char_truncation() {
    let url = "https://example.com/article";
    
    let hash: String = blake3::hash(url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_eq!(hash.len(), 8);
}

#[test]
fn test_blake3_different_urls_different_hashes() {
    let url1 = "https://example.com/article1";
    let url2 = "https://example.com/article2";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_ne!(hash1, hash2);
}

#[test]
fn test_blake3_case_sensitive() {
    let url1 = "https://example.com/Article";
    let url2 = "https://example.com/article";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_ne!(hash1, hash2);
}

#[test]
fn test_blake3_hash_with_query_params() {
    let url1 = "https://example.com/article";
    let url2 = "https://example.com/article?utm_source=test";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    // Different URLs should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_blake3_hash_with_fragment() {
    let url1 = "https://example.com/article";
    let url2 = "https://example.com/article#section";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    // Different URLs should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_blake3_hash_with_trailing_slash() {
    let url1 = "https://example.com/article";
    let url2 = "https://example.com/article/";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    // Different URLs should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_blake3_hash_hexadecimal_output() {
    let url = "https://example.com";
    
    let hash: String = blake3::hash(url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    // Should only contain hex characters (0-9, a-f)
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_blake3_hash_empty_string() {
    let hash: String = blake3::hash(b"")
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_eq!(hash.len(), 8);
}

#[test]
fn test_blake3_hash_very_long_url() {
    let long_url = format!("https://example.com/{}", "a".repeat(10000));
    
    let hash: String = blake3::hash(long_url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_eq!(hash.len(), 8);
}

#[test]
fn test_blake3_hash_with_unicode() {
    let url = "https://example.com/文章/статья";
    
    let hash: String = blake3::hash(url.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    assert_eq!(hash.len(), 8);
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_blake3_hash_deterministic_across_runs() {
    let url = "https://example.com/test";
    
    // Generate hash multiple times
    let hashes: Vec<String> = (0..5)
        .map(|_| {
            blake3::hash(url.as_bytes())
                .to_hex()
                .chars()
                .take(8)
                .collect()
        })
        .collect();
    
    // All should be identical
    let first = &hashes[0];
    assert!(hashes.iter().all(|h| h == first));
}

#[test]
fn test_blake3_hash_http_vs_https() {
    let url1 = "http://example.com/article";
    let url2 = "https://example.com/article";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    // Different protocols should produce different hashes
    assert_ne!(hash1, hash2);
}

#[test]
fn test_blake3_hash_www_vs_no_www() {
    let url1 = "https://www.example.com/article";
    let url2 = "https://example.com/article";
    
    let hash1: String = blake3::hash(url1.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    let hash2: String = blake3::hash(url2.as_bytes())
        .to_hex()
        .chars()
        .take(8)
        .collect();
    
    // Different URLs should produce different hashes
    assert_ne!(hash1, hash2);
}

// Note: In actual implementation, URL canonicalization should be done
// BEFORE hashing to ensure that equivalent URLs produce the same hash.
// These tests verify the hash function behavior on the raw input.

