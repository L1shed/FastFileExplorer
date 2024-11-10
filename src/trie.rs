use std::{collections::HashMap, time::SystemTime};

#[derive(Debug)]
pub struct SearchResult {
    pub path: String,
    pub is_file: bool,
    pub size: u64,
    pub modified: SystemTime,
}

#[derive(Debug)]
pub struct TrieNode {
    is_file: bool,
    children: HashMap<String, TrieNode>,
    size: u64,
    modified: SystemTime,
    full_path: String,
}

impl TrieNode {
    pub fn new(is_file: bool, size: u64, modified: SystemTime, full_path: String) -> Self {
        TrieNode {
            is_file,
            children: HashMap::new(),
            size,
            modified,
            full_path,
        }
    }

    pub fn insert(&mut self, path_components: &[String], is_file: bool, size: u64, modified: SystemTime, full_path: String) {
        if path_components.is_empty() {
            return;
        }

        let component = &path_components[0];
        let child = self.children.entry(component.clone()).or_insert_with(|| {
            TrieNode::new(
                path_components.len() == 1 && is_file,
                if path_components.len() == 1 { size } else { 0 },
                modified,
                full_path.clone(),
            )
        });

        if path_components.len() > 1 {
            child.insert(&path_components[1..], is_file, size, modified, full_path);
        }
    }

    pub fn search_pattern(&self, pattern: &str, results: &mut Vec<SearchResult>) {
        if self.full_path.contains(pattern) {
            results.push(SearchResult {
                path: self.full_path.clone(),
                is_file: self.is_file,
                size: self.size,
                modified: self.modified,
            });
        }

        for child in self.children.values() {
            child.search_pattern(pattern, results);
        }
    }
}