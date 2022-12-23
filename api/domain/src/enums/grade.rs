use serde::{Deserialize, Serialize};

/// Users achieved grade
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Grade {
    /// Grade: 1-100%
    Percentage(u8),
    /// Grade: 1-7
    Scale(u8),
}
