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

    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::DarkFg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.activeForeground",
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule("sideBar.background", palette.base(BaseScale::DarkBg));

    builder.add_workspace_rule("activityBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::FadedFg));
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Semantic("function.attribute"),
            Semantic("boolean"),
        ],
        (palette.pink(), FontStyle::Bold),
    );

    builder.add_rules(
        &[
            Semantic("variable"),
            Semantic("parameter"),
            Semantic("function"),
            Semantic("method"),
            Semantic("property"),
            Semantic("enumMember"),
        ],
        palette.teal(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("typeParameter"),
        ],
        palette.light_teal(),
    );

    builder.add_rules(
        &[
            Semantic("variable.library"),
            Semantic("parameter.library"),
            Semantic("function.library"),
            Semantic("method.library"),
            Semantic("arithmetic"),
            Semantic("bitwise"),
            Semantic("logical"),
            Semantic("comparison"),
            Semantic("property.library"),
            Semantic("enumMember.library"),
        ],
        palette.purple(),
    );

    builder.add_rules(
        &[
            Semantic("type.library"),
            Semantic("class.library"),
            Semantic("struct.library"),
            Semantic("enum.library"),
            Semantic("union.library"),
            Semantic("typeAlias.library"),
            Semantic("builtinType"),
        ],
        palette.light_purple(),
    );

    builder.add_rules(
        &[
            Semantic("variable.declaration"),
            Semantic("parameter.declaration"),
            Semantic("function.declaration"),
            Semantic("method.declaration"),
            Semantic("property.declaration"),
            Semantic("enumMember.declaration"),
        ],
        palette.blue(),
    );

    builder.add_rules(
        &[
            Semantic("type.declaration"),
            Semantic("class.declaration"),
            Semantic("struct.declaration"),
            Semantic("enum.declaration"),
            Semantic("union.declaration"),
            Semantic("typeAlias.declaration"),
            Semantic("typeParameter.declaration"),
        ],
        palette.light_blue(),
    );

    builder.add_rules(
        &[
            Semantic("variable.trait"),
            Semantic("function.trait"),
            Semantic("method.trait"),
        ],
        palette.green(),
    );
    builder.add_rules(
        &[
            Semantic("interface"),
            Semantic("type.trait"),
            Semantic("class.trait"),
            Semantic("struct.trait"),
            Semantic("enum.trait"),
            Semantic("union.trait"),
            Semantic("typeAlias.trait"),
            Semantic("typeParameter.trait"),
        ],
        palette.light_green(),
    );

    builder.add_rules(
        &[Semantic("number"), Semantic("characterLiteral")],
        palette.yellow(),
    );

    builder.add_rule(Semantic("string"), palette.red());

    builder.add_rule(Semantic("macro"), palette.orange());

    builder.add_rule(Semantic("lifetime"), (palette.orange(), FontStyle::Italic));

    builder.add_rule(Semantic("comment"), FontStyle::Bold);
}
