use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AnnouncementId(i64);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Announcement {
    pub id: AnnouncementId,
    pub title: String,
    pub link: String,
    pub summary: String,
    pub created_at: String,
}

impl std::fmt::Display for AnnouncementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
