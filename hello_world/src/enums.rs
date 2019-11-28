use core::fmt;

#[derive(Debug)]
pub enum Type {
    MAGIC,
    FIRE,
    WATER,
    AIR,
    GROUND,
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}