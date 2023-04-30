pub enum ModalAction {
    Close,
    KeepOpen,
}

#[derive(PartialEq)]
pub enum MainTab {
    Overview,
    Characters,
    Notes,
    Settings,
    Locations,
}

impl Default for MainTab {
    fn default() -> Self {
        MainTab::Overview
    }
}
