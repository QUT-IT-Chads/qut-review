#[derive(Debug, Deserialize, Serialize)]
pub enum Semester {
    Normal(u8),
    Summer,
    Other,
}
