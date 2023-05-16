pub mod character;
pub mod ability_scores;
pub mod campaign;
pub mod note;
pub mod aliases;
pub mod campaign_fs;
pub mod race;
pub mod data;
pub mod iterators;
pub mod skill_modifiers;
pub mod extra_stats;
pub mod saving_throws;

pub use character::*;
pub use ability_scores::*;
pub use note::*;
pub use campaign::*;
pub use aliases::*;
pub use campaign_fs::*;
pub use race::*;
pub use data::*;
pub use iterators::*;
pub use skill_modifiers::*;
pub use extra_stats::*;
pub use saving_throws::*;


trait HelperExt<T> {
    fn apply_own(self, func: impl FnOnce(T) -> T) -> T;
}

impl<T> HelperExt<T> for T {
    #[inline(always)]
    fn apply_own(self, func: impl FnOnce(T) -> T) -> T {
        func(self)
    }
}