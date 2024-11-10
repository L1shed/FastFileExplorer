use std::fs;
use std::path::Path;
use std::time::SystemTime;
use crate::trie::{SearchResult, TrieNode};
use crate::Filter;

pub struct FileExplorer {
    root: TrieNode,
}

impl FileExplorer {
    pub fn new() -> Self {
        FileExplorer {
            root: TrieNode::new(false, 0, SystemTime::now(), String::from(".")),
        }
    }

    pub fn build_from_path<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        self.traverse_directory(path)
    }

    fn traverse_directory<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let path_clone = path.as_ref().to_path_buf();
        let read_dir = match fs::read_dir(path) {
            Ok(dir) => dir,
            Err(e) => {
                eprintln!("Error accessing directory with path {}: {}", path_clone.display(), e);
                return Ok(());
            }
        };

        for entry in read_dir {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Error accessing entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error accessing metadata for {:?}: {}", path, e);
                    continue;
                }
            };

            let components: Vec<String> = path
                .components()
                .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
                .collect();

            self.root.insert(
                &components,
                metadata.is_file(),
                metadata.len(),
                metadata.modified().unwrap_or(SystemTime::now()),
                path.to_string_lossy().into_owned(),
            );

            if metadata.is_dir() {
                let _ = self.traverse_directory(path); // Ignore errors from subdirectories
            }
        }
        Ok(())
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let (filter, pattern) = Filter::parse(query);
        let mut results = Vec::new();
        self.root.search_pattern(&pattern, &mut results);
        results.retain(|result| filter.matches(result));
        results
    }
}