use std::path::{Path, PathBuf};

pub fn search_directory_only_epubs(dir_path: impl AsRef<Path>) -> Vec<PathBuf> {
    search_directory(dir_path, |path| path.extension().map_or(false, |e| e.eq_ignore_ascii_case("epub")))
}
pub fn search_directory(dir_path: impl AsRef<Path>, filter: impl Fn(&Path) -> bool) -> Vec<PathBuf> {
    let dir_path = dir_path.as_ref();
    
    let mut pending = vec![dir_path.to_path_buf()];
    let mut out = Vec::new();
    
    while let Some(path) = pending.pop() {
        // println!("[Search Result | {:?}]: {:?}", path.display(), _search_dir(&path));
        let search_results = _search_dir(&path);
        for file in search_results.files{
            if filter(&file) {
                out.push(file);
            }
        }
        for dir in search_results.dirs {
            pending.push(dir);
        }
    }
    
    out
}

fn _search_dir(path: impl AsRef<Path>) -> DirSearchResult {
    let path = path.as_ref();
    
    if !path.exists() || !path.is_dir() {
        return DirSearchResult {
            files: Vec::new(),
            dirs: Vec::new(),
        };
    }
    
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    
    for entry in path.read_dir() {
        for e in entry {
            match e {
                Err(e) => continue,
                Ok(entry) => {
                    let entry_path = entry.path();
                    if entry_path.is_file() {
                        files.push(entry_path);
                    } else if entry_path.is_dir() {
                        dirs.push(entry_path);
                    }
                }
            }
        }
    }
    
    DirSearchResult { files, dirs }
}

#[derive(Debug, Clone)]
pub struct DirSearchResult {
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}
