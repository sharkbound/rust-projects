use std::env;
use eframe::egui::Context;
use dndlib::{CampaignLoadError, load_campaign};
use crate::MainApp;

pub(crate) fn set_new_file_dialog(app: &mut MainApp) {
    let mut file_dialog = egui_file::FileDialog::open_file(Some(env::current_dir().unwrap()))
        .filter(Box::new(|path| match path.extension() {
            Some(extension) => {
                extension.to_string_lossy().eq_ignore_ascii_case("json")
            }
            None => false,
        }));

    file_dialog.open();
    app.file_dialog = Some(file_dialog);
}

pub(crate) fn validate_load_file_selection(app: &mut MainApp, ctx: &Context) -> Result<(), CampaignLoadError> {
    match &mut app.file_dialog {
        None => Ok(()),
        Some(file_dialog) => {
            if !file_dialog.show(ctx).selected() {
                return Ok(());
            }
            match file_dialog.path() {
                None => Ok(()),
                Some(file) => {
                    app.file_dialog = None;
                    match load_campaign(file.as_path()) {
                        Ok(campaign) => {
                            app.campaign = Some(campaign);
                            Ok(())
                        }
                        Err(e) => {
                            Err(e)
                        }
                    }
                }
            }
        }
    }
}
