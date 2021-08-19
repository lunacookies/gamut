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
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rule(
        "editorLineNumber.foreground",
        palette.base(BaseScale::DarkFg),
    );
    builder.add_workspace_rule(
        "editorLineNumber.activeForeground",
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::BrightBg),
    );

    builder.add_workspace_rule("sideBar.background", palette.base(BaseScale::DarkBg));

    builder.add_workspace_rule("activityBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("tab.activeForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("tab.inactiveForeground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("tab.inactiveBackground", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule(
        "editorGroupHeader.tabsBackground",
        palette.base(BaseScale::DarkBg),
    );
    builder.add_workspace_rule(
        "editorGroupHeader.noTabsBackground",
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rule("list.focusBackground", palette.ui_blue());
    builder.add_workspace_rules(
        &[
            "list.inactiveFocusBackground",
            "list.inactiveSelectionBackground",
        ],
        Palette::TRANSPARENT,
    );
    builder.add_workspace_rules(
        &["list.focusForeground", "list.activeSelectionForeground"],
        palette.base(BaseScale::Fg),
    );
    builder.add_workspace_rule("list.hoverBackground", Palette::TRANSPARENT);
    builder.add_workspace_rule("list.focusOutline", Palette::TRANSPARENT);

    builder.add_workspace_rule("editorWidget.background", palette.base(BaseScale::LightBg));

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::FadedFg));

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );
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

    builder.add_rule(
        Semantic("unresolvedReference"),
        (palette.red(), FontStyle::Underline),
    );

    builder.add_rule(Textmate("markup.heading"), FontStyle::Bold);
    builder.add_rules(
        &[
            Textmate("fenced_code.block.language"),
            Textmate("punctuation.definition.bold.markdown"),
            Textmate("punctuation.definition.constant.markdown"),
            Textmate("punctuation.definition.heading.markdown"),
            Textmate("punctuation.definition.italic.markdown"),
            Textmate("punctuation.definition.list.markdown"),
            Textmate("punctuation.definition.markdown"),
            Textmate("punctuation.definition.metadata.markdown"),
            Textmate("punctuation.definition.quote.begin.markdown"),
            Textmate("punctuation.definition.quote.markdown"),
            Textmate("punctuation.definition.raw.markdown"),
            Textmate("punctuation.definition.string.begin.markdown"),
            Textmate("punctuation.definition.string.end.markdown"),
            Textmate("punctuation.separator.key-value.markdown"),
        ],
        palette.teal(),
    );
    builder.add_rules(
        &[
            Textmate("markup.inline.raw.string.markdown"),
            Textmate("markup.fenced_code.block.markdown"),
        ],
        palette.light_purple(),
    );
}
