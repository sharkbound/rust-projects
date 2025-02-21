use crate::directory_utils::search_directory::{search_directory, search_directory_only_epubs};
use std::fs;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

mod directory_utils;
mod epub_utils;

fn main() {
    println!("This currently does not well due to deleted file sticking around. Also has hardcoded deletion/extraction directory paths.");
    // fs::remove_dir_all(r#"C:\RandomCaches\EpubTextSearchExtractionCache\"#);
    return;
    println!("Searching for EPUB files in D:\\media\\ebooks...");

    let epub_files = search_directory_only_epubs(r#"D:\media\ebooks\"#);

    print!("Enter search query: ");
    stdout().flush();
    let mut query = String::new();
    stdin().read_line(&mut query);
    query = query.trim().to_lowercase();
    
    println!("Searching for EPUB files containing the query '{}'...", query);

    for epub_file_path in epub_files {
        // println!("Processing EPUB file: {}", epub_file_path.display());

        let mut epub = match File::open(epub_file_path.clone()) {
            Err(_) => continue,
            Ok(file) => match zip::ZipArchive::new(file) {
                Err(_) => continue,
                Ok(zip_reader) => zip_reader,
            },
        };

        let extraction_dir = PathBuf::from(r#"C:\RandomCaches\EpubTextSearchExtractionCache\"#);
        match epub.extract(extraction_dir.clone()) {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "Error extracting EPUB file [{:?}]: {}",
                    epub_file_path.display(),
                    err
                );
            }
        };

        let xhtml_files = search_directory(&extraction_dir, |p| {
            p.extension().map_or(false, |ext| {
                ext.eq_ignore_ascii_case("xhtml") || ext.eq_ignore_ascii_case("html")
            })
        });

        // println!("search: {:?}", xhtml_files);

        for file in xhtml_files {
            let mut text = String::new();
            match File::open(file.clone()) {
                Err(_) => continue,
                Ok(mut file) => file.read_to_string(&mut text),
            };
            if text.to_ascii_lowercase().contains(&query) {
                println!("MATCH! Found in: {} | Subfile name: {:?}", epub_file_path.display(), file);
                break;
            }
        }

        fs::remove_dir_all(extraction_dir.clone());
        fs::create_dir(extraction_dir.clone());
    }
}
