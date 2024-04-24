
pub trait Data {
    fn create_from_id(id: u64) -> Self;
}
pub struct Project {
    id: u64,
}

impl Data for Project {
    fn create_from_id(id: u64) -> Self {
        Project { id: 0 }
    }
}

pub struct Experience {
    id: u64,
}

impl Data for Experience {
    fn create_from_id(id: u64) -> Self {
        Experience { id: 0 }
    }
}

