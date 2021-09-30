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
        "editorOverviewRuler.border",
        palette.base(BaseScale::BrightBg),
    );

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::BrightBg),
    );

    builder.add_workspace_rule("sideBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("sideBar.border", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rule("activityBar.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("activityBar.foreground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("activityBar.border", palette.base(BaseScale::DarkestBg));
    builder.add_workspace_rule(
        "activityBar.inactiveForeground",
        palette.base(BaseScale::DarkFg),
    );

    builder.add_workspace_rule("tab.activeForeground", palette.base(BaseScale::Fg));
    builder.add_workspace_rule("tab.activeBorderTop", palette.ui_blue());
    builder.add_workspace_rule("tab.inactiveForeground", palette.base(BaseScale::DarkFg));
    builder.add_workspace_rule("tab.inactiveBackground", palette.base(BaseScale::DarkerBg));
    builder.add_workspace_rule(
        "editorGroupHeader.tabsBackground",
        palette.base(BaseScale::DarkerBg),
    );
    builder.add_workspace_rule(
        "editorGroupHeader.noTabsBackground",
        palette.base(BaseScale::LightBg),
    );
    builder.add_workspace_rule("editorGroup.border", palette.base(BaseScale::DarkestBg));

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

    builder.add_workspace_rule("editorWidget.background", palette.base(BaseScale::DarkBg));
    builder.add_workspace_rule("editorWidget.border", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rule("panel.border", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rule("statusBar.background", palette.base(BaseScale::DarkerBg));
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::FadedFg));
    builder.add_workspace_rule("statusBar.border", palette.base(BaseScale::DarkestBg));

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Textmate("keyword"),
            Textmate("storage"),
            Textmate("variable.language.this"),
            // sizeof() in C
            Textmate("keyword.operator.sizeof"),
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
            Textmate("variable"),
            Textmate("constant"),
            Textmate("entity"),
            Textmate("support.type.property-name"),
        ],
        palette.teal(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("enum"),
            Semantic("interface"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("typeParameter"),
            Textmate("entity.name.type"),
            Textmate("storage.type.java"),
            Textmate("storage.type.cs"),
        ],
        palette.light_teal(),
    );

    builder.add_rules(
        &[
            Semantic("variable.library"),
            Semantic("parameter.library"),
            Semantic("function.library"),
            Semantic("method.library"),
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
            Semantic("interface.library"),
            Semantic("union.library"),
            Semantic("typeAlias.library"),
            Semantic("annotation"),
        ],
        palette.light_purple(),
    );

    builder.add_rules(
        &[
            Semantic("variable.defaultLibrary"),
            Semantic("parameter.defaultLibrary"),
            Semantic("function.defaultLibrary"),
            Semantic("method.defaultLibrary"),
            Semantic("arithmetic"),
            Semantic("bitwise"),
            Semantic("logical"),
            Semantic("comparison"),
            Semantic("operator.controlFlow"), // ?
            Semantic("property.defaultLibrary"),
            Semantic("enumMember.defaultLibrary"),
            Semantic("boolean"),
            Textmate("constant.language"),
            Textmate("support.function"),
            Textmate("keyword.operator.arithmetic"),
            Textmate("keyword.operator.increment"),
            Textmate("keyword.operator.decrement"),
            Textmate("keyword.operator.increment-decrement"),
            Textmate("keyword.operator.bitwise"),
            Textmate("keyword.operator.logical"),
            Textmate("keyword.operator.comparison"),
        ],
        palette.green(),
    );

    builder.add_rules(
        &[
            Semantic("type.defaultLibrary"),
            Semantic("class.defaultLibrary"),
            Semantic("struct.defaultLibrary"),
            Semantic("enum.defaultLibrary"),
            Semantic("interface.defaultLibrary"),
            Semantic("union.defaultLibrary"),
            Semantic("typeAlias.defaultLibrary"),
            Semantic("builtinType"),
            Textmate("keyword.type"),
            Textmate("storage.type.primitive"),
            Textmate("storage.type.built-in"),
            Textmate("entity.name.type.numeric"),
            Textmate("entity.name.type.primitive"),
            Textmate("storage.type.numeric.go"),
            Textmate("storage.type.byte.go"),
            Textmate("storage.type.boolean.go"),
            Textmate("storage.type.string.go"),
            Textmate("storage.type.uintptr.go"),
            Textmate("storage.type.error.go"),
            Textmate("storage.type.rune.go"),
            Textmate("storage.type.annotation"),
        ],
        palette.light_green(),
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
            Semantic("interface.declaration"),
            Semantic("union.declaration"),
            Semantic("typeAlias.declaration"),
            Semantic("typeParameter.declaration"),
        ],
        palette.light_blue(),
    );

    builder.add_rules(
        &[
            Semantic("number"),
            Semantic("character"),
            Textmate("constant.numeric"),
        ],
        palette.yellow(),
    );

    builder.add_rules(
        &[
            Semantic("string"),
            Textmate("string"),
            Textmate("punctuation.definition.string"),
        ],
        palette.red(),
    );

    builder.add_rules(
        &[
            Semantic("namespace"),
            Textmate("entity.name.type.namespace"),
            Textmate("storage.modifier.import"),
            Textmate("storage.modifier.package"),
        ],
        palette.base(BaseScale::Fg),
    );

    builder.add_rules(
        &[
            Semantic("attribute.attribute"),
            Semantic("macro.attribute"),
            Semantic("generic.attribute"),
        ],
        palette.base(BaseScale::Fg),
    );

    builder.add_rules(
        &[
            Semantic("formatSpecifier"),
            Semantic("escapeSequence"),
            Textmate("constant.character.escape"),
            Textmate("punctuation.definition.interpolation"),
            Textmate("constant.other.placeholder"),
        ],
        palette.base(BaseScale::Fg),
    );

    builder.add_rule(Semantic("macro"), palette.orange());

    builder.add_rule(Semantic("lifetime"), (palette.orange(), FontStyle::Italic));

    builder.add_rules(&[Semantic("comment"), Textmate("comment")], FontStyle::Bold);

    builder.add_rules(
        &[
            Textmate("punctuation"),
            Textmate("keyword.operator"),
            // closure parameter ‘|’s are highlighted
            // as a binary ‘or’ without this
            Textmate("keyword.operator.logical.rust"),
            Textmate("storage.modifier.pointer"),
            Textmate("storage.modifier.array"),
        ],
        palette.base(BaseScale::Fg),
    );

    builder.add_rule(Semantic("*.mutable"), FontStyle::Underline);

    builder.add_rule(
        Semantic("unresolvedReference"),
        (palette.red(), FontStyle::Underline),
    );

    builder.add_rule(
        Textmate("entity.name.section.markdown"),
        (palette.base(BaseScale::Fg), FontStyle::Bold),
    );
    builder.add_rules(
        &[
            Textmate("fenced_code.block.language"),
            Textmate("punctuation.definition.bold.markdown"),
            Textmate("punctuation.definition.constant.markdown"),
            Textmate("punctuation.definition.constant.begin.markdown"),
            Textmate("punctuation.definition.constant.end.markdown"),
            Textmate("punctuation.definition.heading.markdown"),
            Textmate("punctuation.definition.italic.markdown"),
            Textmate("punctuation.definition.list.begin.markdown"),
            Textmate("punctuation.definition.markdown"),
            Textmate("punctuation.definition.metadata.markdown"),
            Textmate("punctuation.definition.quote.begin.markdown"),
            Textmate("punctuation.definition.quote.markdown"),
            Textmate("punctuation.definition.raw.markdown"),
            Textmate("punctuation.definition.string.begin.markdown"),
            Textmate("punctuation.definition.string.end.markdown"),
            Textmate("punctuation.separator.key-value.markdown"),
            Textmate("markup.heading.setext"),
        ],
        palette.teal(),
    );
    builder.add_rules(
        &[
            Textmate("markup.inline.raw.string.markdown"),
            Textmate("markup.fenced_code.block.markdown"),
            Textmate("markup.raw.block.markdown"),
        ],
        palette.light_purple(),
    );
    builder.add_rules(
        &[
            Textmate("string.other.link.title.markdown"),
            Textmate("constant.other.reference.link.markdown"),
        ],
        palette.base(BaseScale::Fg),
    );
    builder.add_rule(Textmate("markup.italic.markdown"), FontStyle::Italic);
    builder.add_rule(Textmate("markup.bold.markdown"), FontStyle::Bold);
}
