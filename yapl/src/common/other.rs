#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum BracketType {
    Round,
    Square,
    Curly,
}
