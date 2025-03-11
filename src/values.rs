#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CssValue {
    String(String),
    Number(f32),
    Keyword(String),
    Unit(f32, String),
}