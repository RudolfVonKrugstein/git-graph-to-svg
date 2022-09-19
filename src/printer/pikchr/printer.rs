use std::io::Cursor;
use crate::model::model::Model;
use crate::options::layout::{LayoutDirection, LayoutOptions};

pub fn print_pikchr(model: &Model, options: &LayoutOptions) -> Result<String, String> {
  let buf = Cursor::new(Vec::new());
  // Start with the direction
  match options.graph_direction {
    LayoutDirection::UP => write!(buf, "down\n"),
    LayoutDirection::RIGHT => {}
  }
  return Ok(String::from_utf8(buf.into_inner()).map_err(|e| e.to_string())?);
}
