mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let palette = palette::Palette;

    let mut gamut = ThemeBuilder::new("Gamut".to_string(), Type::Dark);
    imp::add_rules(&mut gamut, &palette);
    gamut.build().save()?;

    Ok(())
}
