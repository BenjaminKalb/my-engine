use std::fmt::{self, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
    generation: u32,
}

impl Entity {
    #[inline]
    pub const fn new(id: u32, generation: u32) -> Self {
        Self { id, generation }
    }

    #[inline]
    pub const fn id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub const fn generation(&self) -> u32 {
        self.generation
    }
}

impl fmt::Display for Entity {
    // compact format: "id:generation"
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id, self.generation)
    }
}