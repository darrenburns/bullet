#[derive(Default, Debug, Clone)]
pub struct LineBuffer {
  pub content: String,
  pub is_dirty: bool
}
