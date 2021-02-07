mod imp;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    let mut gamut = ThemeBuilder::new("Gamut".to_string(), Type::Dark);
    imp::add_rules(&mut gamut);
    gamut.build().save()?;

    Ok(())
}
