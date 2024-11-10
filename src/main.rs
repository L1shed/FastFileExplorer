use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

mod helper;
mod trie;
mod file_explorer;
use file_explorer::FileExplorer;
use helper::{format_size, format_size_from_string, format_time};
use trie::SearchResult;

struct Filter {
    is_file: Option<bool>,
    extension: Option<String>,
    min_size: Option<u64>,
    max_size: Option<u64>,
    year: Option<i32>,
}

impl Filter {
    fn new() -> Self {
        Filter {
            is_file: None,
            extension: None,
            min_size: None,
            max_size: None,
            year: None,
        }
    }

    fn parse(query: &str) -> (Self, String) {
        let mut filter = Filter::new();
        let mut search_term = String::new();
        
        for part in query.split_whitespace() {
            if part.starts_with('@') {
                match part {
                    "@file" => filter.is_file = Some(true),
                    "@dir" | "@folder" => filter.is_file = Some(false),
                    _ if part.starts_with("@ext:") => {
                        filter.extension = part[5..].to_string().into();
                    }
                    _ if part.starts_with("@size") => {
                        if part.len() < 7 {
                            continue;
                        }

                        if let Some(size) = format_size_from_string(&part[6..]) {
                            let sign = part.chars().nth(5).unwrap();

                            if sign == '>' {
                                filter.min_size = Some(size);
                            } else if sign == '<' {
                                filter.max_size = Some(size);
                            }
                        }
                    }
                    _ if part.starts_with("@date:") => {
                        if let Ok(year) = part[6..].parse::<i32>() {
                            filter.year = Some(year);
                        }
                    }
                    _ => search_term.push_str(part),
                }
            } else {
                if !search_term.is_empty() {
                    search_term.push(' ');
                }
                search_term.push_str(part);
            }
        }
        
        (filter, search_term)
    }

    fn matches(&self, result: &SearchResult) -> bool {
        // Check file/directory filter
        if let Some(is_file) = self.is_file {
            if result.is_file != is_file {
                return false;
            }
        }

        // Check extension filter
        if let Some(ref ext) = self.extension {
            if !result.is_file || !Path::new(&result.path)
                .extension()
                .and_then(|e| e.to_str())
                .map_or(false, |e| e.eq_ignore_ascii_case(ext)) {
                return false;
            }
        }

        // Check size filter
        if let Some(min_size) = self.min_size {
            if !result.is_file || result.size < min_size {
            return false;
            }
        }
        if let Some(max_size) = self.max_size {
            if !result.is_file || result.size > max_size {
            return false;
            }
        }

        // Check year filter
        if let Some(year) = self.year {
            if let Ok(modified) = result.modified.duration_since(SystemTime::UNIX_EPOCH) {
                let file_year = (1970 + (modified.as_secs() / 31536000)) as i32;
                if file_year != year {
                    return false;
                }
            }
        }

        true
    }
}



fn run_repl(explorer: &FileExplorer) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("\n> ");
        stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let results = explorer.search(input);

        if results.is_empty() {
            println!("No matches found for '{}'", input);
        } else {
            println!("\nFound {} matches for '{}':", results.len(), input);
            println!("\n{:<60} {:<8} {:<12} {}", "Path", "Type", "Size", "Modified");
            println!("{}", "-".repeat(100));

            for result in results {
                println!("{:<60} {:<8} {:<12} {}",
                    result.path,
                    if result.is_file { "File" } else { "Dir" },
                    if result.is_file { format_size(result.size) } else { "-".to_string() },
                    format_time(result.modified)
                );
            }
        }
    }
}

fn main() -> io::Result<()> {
    println!("Building file index...");
    let mut explorer = FileExplorer::new();
    explorer.build_from_path("C:/")?;
    println!("File index built successfully!");
    
    println!("\nWelcome to Fast File Explorer!");
    println!("Type a file name to start searching");

    run_repl(&explorer)?;

    Ok(())
}