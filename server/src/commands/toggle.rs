use anyhow::Result;

use crate::marks;

pub fn run(text: &str) -> Result<()> {
    let path = marks::marks_path()?;
    marks::toggle(&path, text)
}
