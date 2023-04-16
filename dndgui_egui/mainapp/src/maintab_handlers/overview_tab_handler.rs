use eframe::egui::{Context, Ui};
use crate::MainApp;

pub(crate) fn render_maintab_overview(ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    if app.campaign.is_none() {
        ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
        return;
    }
    let campaign = app.campaign.as_ref().unwrap();
}