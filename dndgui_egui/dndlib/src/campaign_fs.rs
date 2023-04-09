use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use serde_json;

use crate::{DndCampaign};

pub fn load_campaign(path: &Path) -> Option<DndCampaign> {
    if !path.exists() {
        return None;
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => return None
    };

    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(_) => return None,
        _ => {}
    }

    Some(match serde_json::from_str(&file_contents) {
        Ok(campaign) => campaign,
        Err(_) => return None
    })
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
                    None => continue,
                    Some(campaign) => campaigns.push(campaign),
                }
            }
            _ => {}
        }
    }

    campaigns
}