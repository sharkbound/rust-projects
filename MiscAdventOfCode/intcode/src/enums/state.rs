#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Paused,
    WaitingForInput,
    Running,
    Halted,
    NotStarted,
}
