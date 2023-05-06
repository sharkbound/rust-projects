use eframe::egui;
use eframe::egui::{Context, Ui};
use dndlib::{AttributeStats, Character, DndCampaign, Note, Race};
use crate::{file_dialog_handler, MainApp};

pub(crate) fn show_top_menu(ctx: &Context, app: &mut MainApp) {
    egui::TopBottomPanel::top("primary_topbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("Load", |ui| {
                if ui.button("load campaign from file").clicked() {
                    file_dialog_handler::set_new_file_dialog(app);
                    ui.close_menu();
                }
            });

            if ui.button("Load Example Campaign").clicked() {
                app.campaign = Some(create_filler_campaign());
            }
        })
    });
}

fn create_filler_campaign() -> DndCampaign {
    DndCampaign::new(
        "Example Campaign For Testing",
        vec![
                Character::new("Default Dan", Race::BugBear, AttributeStats::new(
                    16,
                    10,
                    8,
                    20,
                    15,
                    16,
                ), Some(14)).edit(|chr| {
                    chr.edit_note(|note| {
                        note.edit_title(|_| "Default Dan's Note!".to_owned());
                        note.edit_content(|_| "Default Dan's Note Content! Not much to see here, cause Default Dan is a pretty generic guy!".to_owned());
                    });
                }),
                Character::new("John Doe", Race::Human, AttributeStats::new(
                    40,
                    10,
                    80,
                    20,
                    45,
                    1337,
                ), Some(14)).edit(|chr| {
                    chr.edit_note(|note| {
                        note.edit_title(|_| "John Doe's Note!".to_owned());
                        note.edit_content(|_| "Generic note for John Doe!".to_owned());
                    });
                })
            ],
        vec![
            Note::new("Default Note For Testing!", "\
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                    Neque vitae tempus quam pellentesque nec nam. Sed blandit libero volutpat sed. Tempus egestas sed sed risus pretium quam. \
                    Sapien faucibus et molestie ac. Blandit libero volutpat sed cras ornare arcu. \
                    Laoreet suspendisse interdum consectetur libero id faucibus nisl tincidunt. \
                    Viverra maecenas accumsan lacus vel facilisis volutpat est. Amet porttitor eget dolor morbi non arcu risus. \
                    Gravida rutrum quisque non tellus orci ac auctor. Sit amet porttitor eget dolor morbi. \
                    Duis tristique sollicitudin nibh sit amet commodo nulla. Iaculis urna id volutpat lacus laoreet. \
                    Enim ut tellus elementum sagittis."),
            Note::new("Secret Note! Dont Open!", "This is a secret note! Dont look at it!"),
        ])
}