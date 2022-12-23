use serde::{Deserialize, Serialize};

/// Users achieved grade
#[derive(Debug, Deserialize, Serialize)]
pub enum Semester {
    Normal(u8),
    Summer,
    Other,
}
