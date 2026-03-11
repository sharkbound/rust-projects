use ratatui_interact::prelude::ClickRegionRegistry;
pub fn create_click_region_registry() -> ClickRegionRegistry<usize> {
    ClickRegionRegistry::new()
}
