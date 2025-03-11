use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::properties;
use crate::values::CssValue;

#[derive(Clone)]
pub struct CssStyleList(Arc<RwLock<CssStyleListInner>>);

struct CssStyleListInner {
    props: HashMap<&'static str, CssValue>,
    parent: Option<CssStyleList>,
}

impl CssStyleList {
    pub(crate) fn root() -> CssStyleList {
        CssStyleList(Arc::new(RwLock::new(CssStyleListInner {
            props: generate_default_style(),
            parent: None,
        })))
    }

    pub(crate) fn create(&self) -> Self {
        CssStyleList(Arc::new(RwLock::new(CssStyleListInner {
            props: HashMap::new(),
            parent: Some(self.clone()),
        })))
    }

    pub(crate) fn set_property(&mut self, property_name: &'static str, value: CssValue) {
        self.0.write().unwrap().props.insert(property_name, value);
    }

    pub(crate) fn get_property(&self, property_name: &'static str) -> Option<CssValue> {
        let binding = self.0.read().unwrap();
        match binding.props.get(property_name) {
            Some(value) => Some(value.clone()),
            None => {
                match &binding.parent {
                    Some(parent) => parent.get_property(property_name),
                    None => None,
                }
            }
        }
    }
}

fn generate_default_style() -> HashMap<&'static str, CssValue> {
    let mut props = HashMap::new();

    props.insert(properties::BACKGROUND, CssValue::Keyword("white".to_string()));
    props.insert(properties::BORDER, CssValue::Keyword("100%".to_string()));
    props.insert(properties::BORDER_BOTTOM, CssValue::Keyword("100%".to_string()));
    props.insert(properties::BORDER_LEFT, CssValue::Keyword("100%".to_string()));
    props.insert(properties::BORDER_RIGHT, CssValue::Keyword("100%".to_string()));
    props.insert(properties::BORDER_TOP, CssValue::Keyword("100%".to_string()));
    props.insert(properties::BORDER_RADIUS, CssValue::Keyword("100%".to_string()));
    props.insert(properties::BORDER_COLOR, CssValue::Keyword("black".to_string()));
    props.insert(properties::BORDER_STYLE, CssValue::Keyword("solid".to_string()));
    props.insert(properties::BORDER_WIDTH, CssValue::Keyword("1px".to_string()));
    props.insert(properties::COLOR, CssValue::Keyword("black".to_string()));
    props.insert(properties::DISPLAY, CssValue::Keyword("block".to_string()));
    props.insert(properties::FONT_FAMILY, CssValue::Keyword("Arial".to_string()));
    props.insert(properties::FONT_SIZE, CssValue::Keyword("16px".to_string()));
    props.insert(properties::FONT_STYLE, CssValue::Keyword("normal".to_string()));
    props.insert(properties::FONT_WEIGHT, CssValue::Keyword("normal".to_string()));
    props.insert(properties::HEIGHT, CssValue::Keyword("100%".to_string()));
    props.insert(properties::LINE_HEIGHT, CssValue::Keyword("1".to_string()));
    props.insert(properties::MARGIN, CssValue::Keyword("0".to_string()));
    props.insert(properties::MARGIN_BOTTOM, CssValue::Keyword("0".to_string()));
    props.insert(properties::MARGIN_LEFT, CssValue::Keyword("0".to_string()));
    props.insert(properties::MARGIN_RIGHT, CssValue::Keyword("0".to_string()));
    props.insert(properties::MARGIN_TOP, CssValue::Keyword("0".to_string()));
    props.insert(properties::PADDING, CssValue::Keyword("0".to_string()));
    props.insert(properties::PADDING_BOTTOM, CssValue::Keyword("0".to_string()));
    props.insert(properties::PADDING_LEFT, CssValue::Keyword("0".to_string()));
    props.insert(properties::PADDING_RIGHT, CssValue::Keyword("0".to_string()));
    props.insert(properties::PADDING_TOP, CssValue::Keyword("0".to_string()));
    props.insert(properties::TEXT_ALIGN, CssValue::Keyword("left".to_string()));
    props.insert(properties::TEXT_DECORATION, CssValue::Keyword("none".to_string()));
    props.insert(properties::TEXT_TRANSFORM, CssValue::Keyword("none".to_string()));
    props.insert(properties::WIDTH, CssValue::Keyword("100%".to_string()));
    props.insert(properties::Z_INDEX, CssValue::Keyword("0".to_string()));

    props
}
