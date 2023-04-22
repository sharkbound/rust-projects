use eframe::egui::{Context, FontFamily, FontId, RichText, Ui};
use crate::MainApp;

pub(crate) fn render_maintab_overview(ctx: &Context, ui: &mut Ui, app: &mut MainApp) {
    if app.campaign.is_none() {
        ui.label("No campaign is loaded, So there is nothing to show here! (well other than this text this is...)");
        return;
    }

    let campaign = app.campaign.as_ref().unwrap();
    // ui.label(RichText::new(campaign.title()).font(FontId::proportional(40.0)));
    ui.label(RichText::new(campaign.title()).font(FontId::new(30.0, FontFamily::Name("dnd".into()))));
}