use std::{fs};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde_json;

use crate::{DndCampaign};

#[derive(Debug)]
pub enum CampaignLoadError {
    FileNotFound(PathBuf),
    Json(PathBuf, serde_json::Error),
    ReadFile(PathBuf),
    FileOpen(PathBuf),
}

impl CampaignLoadError {
    pub fn path(&self) -> &Path {
        match *self {
            CampaignLoadError::FileNotFound(ref path) => path,
            CampaignLoadError::Json(ref path, _) => path,
            CampaignLoadError::ReadFile(ref path) => path,
            CampaignLoadError::FileOpen(ref path) => path,
        }
    }
}

impl Display for CampaignLoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for CampaignLoadError {}


pub fn load_campaign(path: &Path) -> Result<DndCampaign, CampaignLoadError> {
    if !path.exists() {
        return Err(CampaignLoadError::FileNotFound(path.to_path_buf()));
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(CampaignLoadError::FileOpen(path.to_path_buf())),
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(_) => return Err(CampaignLoadError::ReadFile(path.to_path_buf())),
        _ => {}
    }

    match serde_json::from_str(&file_contents) {
        Ok(campaign) => Ok(campaign),
        Err(e) => return Err(CampaignLoadError::Json(path.to_path_buf(), e)),
    }
}

pub fn save_campaign(path: &Path, campaign: &DndCampaign) -> bool {
    let mut file = match File::options().write(true).open(&path) {
        Ok(file) => file,
        Err(_) => return false
    };

    let json = match serde_json::to_string(campaign) {
        Ok(string) => string,
        Err(_) => return false
    };

    file.write_all(json.as_bytes()).is_ok()
}

pub fn load_campaigns(directory: &Path) -> Vec<DndCampaign> {
    let mut campaigns = vec![];
    if !directory.is_dir() || !directory.exists() {
        return campaigns;
    }

    let entries = match fs::read_dir(directory) {
        Err(_) => return campaigns,
        Ok(entries) => entries
    };

    for path in entries.flatten().map(|x| x.path()) {
        if !path.is_file() {
            continue;
        }

        match path.extension() {
            Some(ext) if ext.to_string_lossy().eq_ignore_ascii_case("json") => {
                match load_campaign(&path) {
                    Ok(campaign) => campaigns.push(campaign),
                    Err(_) => continue
                }
            }
            _ => {}
        }
    }

    campaigns
}