use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
  Base,
  Blue,
  Green,
  Yellow,
  Red,
  Purple,
  Pink,
}

impl From<&str> for Color {
  fn from(s: &str) -> Self {
    match s.to_lowercase().as_str() {
      "base" => Color::Base,
      "blue" => Color::Blue,
      "green" => Color::Green,
      "yellow" => Color::Yellow,
      "red" => Color::Red,
      "purple" => Color::Purple,
      "pink" => Color::Pink,
      _ => Color::Base, // Default to Base if unknown
    }
  }
}

impl std::fmt::Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Color::Base => write!(f, "base"),
      Color::Blue => write!(f, "blue"),
      Color::Green => write!(f, "green"),
      Color::Yellow => write!(f, "yellow"),
      Color::Red => write!(f, "red"),
      Color::Purple => write!(f, "purple"),
      Color::Pink => write!(f, "pink"),
    }
  }
}
