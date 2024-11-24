#[derive(Default, Clone, Copy)]
pub struct Style {
    pub heading: bool,
    pub bold: bool,
    pub italic: bool,
    pub code: bool
}

pub enum MarkdownItem {
    Text {
        style: Style,
        content: String
    },
    NewLine,
    CodeBlock {
        style: Style,
        content: String
    },
    BulletPoint {
        style: Style,
        content: String,
    }
}