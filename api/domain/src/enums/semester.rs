use serde::{Deserialize, Serialize};

/// Users achieved grade
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Semester {
    Normal(u8),
}
