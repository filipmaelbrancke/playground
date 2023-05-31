#[derive(Clone, Copy)]
pub enum Owner {
    User,
    Group,
    Other,
}

impl Owner {
    pub fn masks(&self) -> [u32; 3] {
        // return array of mask values (for given owner)
        match self {
            Self::User => [0o400, 0o200, 0o100], // read, write, execute masks for `User`
            Self::Group => [0o040, 0o020, 0o010], // read, write, execute masks for `Group`
            Self::Other => [0o004, 0o002, 0o001], // read, write, execute masks for `Other`
        }
    }
}
