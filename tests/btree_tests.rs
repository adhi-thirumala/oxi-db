use oxi_db::BTree;

#[test]
fn test_btree_insert_and_search() {
    let mut tree = BTree::new();
    
    // Insert key-value pairs
    tree.insert("key1".to_string(), "value1".to_string());
    tree.insert("key2".to_string(), "value2".to_string());
    tree.insert("key3".to_string(), "value3".to_string());
    
    // Search for keys
    assert_eq!(tree.search(&"key1".to_string()), Some(&"value1".to_string()));
    assert_eq!(tree.search(&"key2".to_string()), Some(&"value2".to_string()));
    assert_eq!(tree.search(&"key3".to_string()), Some(&"value3".to_string()));
    assert_eq!(tree.search(&"key4".to_string()), None);
}

#[test]
fn test_btree_remove() {
    let mut tree = BTree::new();
    
    // Insert key-value pairs
    tree.insert("key1".to_string(), "value1".to_string());
    tree.insert("key2".to_string(), "value2".to_string());
    tree.insert("key3".to_string(), "value3".to_string());
    
    // Remove a key
    let removed = tree.remove(&"key2".to_string());
    assert_eq!(removed, Some("value2".to_string()));
    
    // Check that the key is no longer in the tree
    assert_eq!(tree.search(&"key2".to_string()), None);
    
    // Check that other keys are still in the tree
    assert_eq!(tree.search(&"key1".to_string()), Some(&"value1".to_string()));
    assert_eq!(tree.search(&"key3".to_string()), Some(&"value3".to_string()));
}

#[test]
fn test_btree_update() {
    let mut tree = BTree::new();
    
    // Insert key-value pairs
    tree.insert("key1".to_string(), "value1".to_string());
    
    // Update a value
    if let Some(value) = tree.get_mut(&"key1".to_string()) {
        *value = "updated".to_string();
    }
    
    // Check that the value was updated
    assert_eq!(tree.search(&"key1".to_string()), Some(&"updated".to_string()));
}

#[test]
fn test_btree_traversal() {
    let mut tree = BTree::new();
    
    // Insert key-value pairs
    tree.insert("key3".to_string(), "value3".to_string());
    tree.insert("key1".to_string(), "value1".to_string());
    tree.insert("key2".to_string(), "value2".to_string());
    
    // Collect all key-value pairs
    let mut pairs = Vec::new();
    tree.traverse(|k, v| pairs.push((k.clone(), v.clone())));
    
    // Check that pairs are in order
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(
        pairs,
        vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]
    );
}

#[test]
fn test_btree_to_vec() {
    let mut tree = BTree::new();
    
    // Insert key-value pairs
    tree.insert("key3".to_string(), "value3".to_string());
    tree.insert("key1".to_string(), "value1".to_string());
    tree.insert("key2".to_string(), "value2".to_string());
    
    // Get all key-value pairs as a vector
    let mut pairs = tree.to_vec();
    
    // Check that pairs are in order
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(
        pairs,
        vec![
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]
    );
}

#[test]
fn test_btree_is_empty() {
    let mut tree: BTree<String, String> = BTree::new();
    
    // Check that a new tree is empty
    assert!(tree.is_empty());
    
    // Insert a key-value pair
    tree.insert("key1".to_string(), "value1".to_string());
    
    // Check that the tree is no longer empty
    assert!(!tree.is_empty());
    
    // Remove the key-value pair
    tree.remove(&"key1".to_string());
    
    // Check that the tree is empty again
    assert!(tree.is_empty());
}

#[test]
fn test_btree_len() {
    let mut tree = BTree::new();
    
    // Check that a new tree has length 0
    assert_eq!(tree.len(), 0);
    
    // Insert key-value pairs
    tree.insert("key1".to_string(), "value1".to_string());
    assert_eq!(tree.len(), 1);
    
    tree.insert("key2".to_string(), "value2".to_string());
    assert_eq!(tree.len(), 2);
    
    tree.insert("key3".to_string(), "value3".to_string());
    assert_eq!(tree.len(), 3);
    
    // Remove a key-value pair
    tree.remove(&"key2".to_string());
    assert_eq!(tree.len(), 2);
}

#[test]
fn test_btree_clear() {
    let mut tree = BTree::new();
    
    // Insert key-value pairs
    tree.insert("key1".to_string(), "value1".to_string());
    tree.insert("key2".to_string(), "value2".to_string());
    tree.insert("key3".to_string(), "value3".to_string());
    
    // Clear the tree
    tree.clear();
    
    // Check that the tree is empty
    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);
    assert_eq!(tree.search(&"key1".to_string()), None);
}
