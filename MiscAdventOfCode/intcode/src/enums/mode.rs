#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}
