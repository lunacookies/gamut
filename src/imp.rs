use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder) {
    workspace_colors(builder);
    syntax_highlighting(builder);
}

fn workspace_colors(builder: &mut ThemeBuilder) {}

fn syntax_highlighting(builder: &mut ThemeBuilder) {}
