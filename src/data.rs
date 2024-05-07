#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Project {
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Experience {
    pub begin: String,
    pub end: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

impl Experience {
    pub fn new(begin: impl Into<String>, end: impl Into<String>, title: impl Into<String>, description: impl Into<String>, tags: Vec<String>) -> Self {
        Self {
            begin: begin.into(),
            end: end.into(),
            title: title.into(),
            description: description.into(),
            tags,
        }
    }
}