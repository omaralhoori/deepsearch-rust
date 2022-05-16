use std::{error::Error};
use clap::Parser;
use walkdir::WalkDir;
use std::fs;

/// Search deeply in your documents
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The text you are searching for
    #[clap(short, long)]
    query: String,

    /// Directory to start searching from
    #[clap(short='d',long, default_value_t= String::from("."))]
    start_path: String,

    /// Is searching sensitive
    #[clap(short, long)]
    sensitive: bool,

    /// Search only in current or given directory without recursion
    #[clap(short, long)]
    non_recursive: bool,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let paths = read_dir(args.start_path.as_str(), args.non_recursive);
    for path in paths{
        let content = fs::read_to_string(path.clone()).unwrap_or_default();
        let results = if args.sensitive {
            search_case_sensitive(&args.query, &content)
        } else {
            search(&args.query, &content)
        };
        for result in results {
            println!("{} ------> {}", result, path);
        }
    }
    Ok(())
}


pub fn read_dir(dir: &str, non_recursive: bool) -> Vec<String> {
    let mut paths = Vec::new();
    let mut entries_dir =  WalkDir::new(dir);
    if non_recursive {
        entries_dir = WalkDir::new(dir).max_depth(1); 
    }
    let entries = entries_dir.into_iter().filter_map(|e| e.ok());
    for entry in entries {
        if entry.file_type().is_file() {
            paths.push(format!("{}", entry.path().display()));

        }
    }
    paths
}

pub fn search_case_sensitive(query: &str, contents: &str) -> Vec<String> {
    let mut results = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        if line.contains(&query) {
            results.push(format!("{}. {}", idx + 1, line));
        }
    }
    
    results
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.push(format!("{}. {}", idx + 1, line));
        }
    }
    
    results
}
