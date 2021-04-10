use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rule("foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("editor.foreground", palette.base(BaseScale::Fg));

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkerBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::FadedFg));
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rule(Semantic("keyword"), (palette.pink(), FontStyle::Bold));

    builder.add_rules(&[Semantic("function"), Semantic("method")], palette.teal());

    builder.add_rules(
        &[
            Semantic("variable"),
            Semantic("parameter"),
            Semantic("property"),
        ],
        palette.green(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("builtinType"),
            Semantic("interface"),
            Semantic("typeParameter"),
        ],
        palette.purple(),
    );

    builder.add_rules(&[Semantic("enum"), Semantic("enumMember")], palette.blue());
}
