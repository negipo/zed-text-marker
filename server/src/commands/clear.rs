use anyhow::Result;

use crate::marks;

pub fn run() -> Result<()> {
    let path = marks::marks_path()?;
    marks::clear(&path)
}
