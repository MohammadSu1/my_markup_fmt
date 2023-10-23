pub(crate) fn is_component(name: &str) -> bool {
    name.contains('-') || name.contains(|c: char| c.is_ascii_uppercase())
}

static NON_SENSITIVE_TAGS: [&'static str; 63] = [
    "address",
    "blockquote",
    "center",
    "dialog",
    "div",
    "figure",
    "figcaption",
    "footer",
    "form",
    "header",
    "hr",
    "legend",
    "listing",
    "main",
    "p",
    "plaintext",
    "pre",
    "search",
    "xmp",
    "area",
    "base",
    "basefont",
    "datalist",
    "head",
    "link",
    "meta",
    "noembed",
    "noframes",
    "param",
    "rp",
    "title",
    "html",
    "body",
    "article",
    "aside",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "hgroup",
    "nav",
    "section",
    "table",
    "tr",
    "thead",
    "th",
    "tbody",
    "td",
    "tfoot",
    "dir",
    "dd",
    "dl",
    "dt",
    "menu",
    "ol",
    "ul",
    "li",
    "fieldset",
    "video",
    "audio",
    "picture",
];

pub(crate) fn is_whitespace_sensitive_tag(name: &str) -> bool {
    // There's also a tag called "a" in SVG, so we need to check it specially.
    name.eq_ignore_ascii_case("a")
        || !NON_SENSITIVE_TAGS
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(name))
            && !css_dataset::tags::SVG_TAGS
                .iter()
                .any(|tag| tag.eq_ignore_ascii_case(name))
}

static VOID_ELEMENTS: [&'static str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "source", "track",
    "wbr", "param",
];

pub(crate) fn is_void_element(name: &str) -> bool {
    VOID_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(name))
}
