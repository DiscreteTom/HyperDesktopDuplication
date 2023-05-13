use serde_derive::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct DisplaysInfo {
  pub displays: Vec<DisplayInfo>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DisplayInfo {
  pub bottom: i32,
  pub top: i32,
  pub left: i32,
  pub right: i32,
  pub name: String,
  pub rotation: i32,
}
