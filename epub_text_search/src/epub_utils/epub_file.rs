use std::path::{Path, PathBuf};

pub struct EpubFile {
    path: PathBuf,
}

impl EpubFile {
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
    pub fn search(&self) -> Vec<String> {
        if !self.exists() {
            return Vec::new();
        }
        unimplemented!()
    }
}

impl TryFrom<PathBuf> for EpubFile {
    type Error = ();

    fn try_from(value: PathBuf) -> Result<EpubFile, ()> {
        if !value.exists() {
            return Err(());
        }
        if !value
            .extension()
            .filter(|e| {
                e.to_ascii_lowercase().eq_ignore_ascii_case("epub")
                    || e.to_ascii_lowercase().eq_ignore_ascii_case("zip")
            })
            .is_none()
        {
            return Err(());
        }
        Ok(EpubFile { path: value })
    }
}

impl TryFrom<&str> for EpubFile {
    type Error = ();
    fn try_from(value: &str) -> Result<EpubFile, ()> {
        EpubFile::try_from(PathBuf::from(value))
    }
}

impl TryFrom<&Path> for EpubFile {
    type Error = ();
    fn try_from(value: &Path) -> Result<EpubFile, ()> {
        EpubFile::try_from(value.to_path_buf())
    }
}
