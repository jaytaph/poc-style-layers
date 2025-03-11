use std::collections::HashMap;
use crate::layer::CssPropertyListLayer;
use crate::properties;
use crate::values::CssValue;

pub(crate) type LayerId = usize;

pub(crate) const ROOT_LAYER_ID: LayerId = 0;

pub(crate) struct LayerArena {
    layers: HashMap<LayerId, CssPropertyListLayer>,
    next_layer_id: LayerId,
}

impl LayerArena {
    pub(crate) fn new() -> Self {
        let mut arena = Self {
            layers: HashMap::new(),
            next_layer_id: 1,
        };

        arena.layers.insert(ROOT_LAYER_ID, generate_root_layer());
        arena
    }

    pub(crate) fn new_layer(&mut self, parent_layer_id: LayerId) -> LayerId {
        let layer_id = self.next_layer_id;
        self.next_layer_id += 1;
        self.layers.insert(layer_id, CssPropertyListLayer::new(layer_id, parent_layer_id));
        layer_id
    }

    pub(crate) fn get_layer(&self, layer_id: LayerId) -> Option<&CssPropertyListLayer> {
        self.layers.get(&layer_id)
    }

    pub(crate) fn get_layer_mut(&mut self, layer_id: LayerId) -> Option<&mut CssPropertyListLayer> {
        self.layers.get_mut(&layer_id)
    }
}


/// This will generate a list of default CSS properties for the root layer
fn generate_root_layer() -> CssPropertyListLayer {
    let mut root_layer = CssPropertyListLayer::new(ROOT_LAYER_ID, ROOT_LAYER_ID);
    root_layer.set_property(properties::BACKGROUND, CssValue::Keyword("white".to_string()));
    root_layer.set_property(properties::BORDER, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::BORDER_BOTTOM, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::BORDER_LEFT, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::BORDER_RIGHT, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::BORDER_TOP, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::BORDER_RADIUS, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::BORDER_COLOR, CssValue::Keyword("black".to_string()));
    root_layer.set_property(properties::BORDER_STYLE, CssValue::Keyword("solid".to_string()));
    root_layer.set_property(properties::BORDER_WIDTH, CssValue::Keyword("1px".to_string()));
    root_layer.set_property(properties::COLOR, CssValue::Keyword("black".to_string()));
    root_layer.set_property(properties::DISPLAY, CssValue::Keyword("block".to_string()));
    root_layer.set_property(properties::FONT_FAMILY, CssValue::Keyword("Arial".to_string()));
    root_layer.set_property(properties::FONT_SIZE, CssValue::Keyword("16px".to_string()));
    root_layer.set_property(properties::FONT_STYLE, CssValue::Keyword("normal".to_string()));
    root_layer.set_property(properties::FONT_WEIGHT, CssValue::Keyword("normal".to_string()));
    root_layer.set_property(properties::HEIGHT, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::LINE_HEIGHT, CssValue::Keyword("1".to_string()));
    root_layer.set_property(properties::MARGIN, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::MARGIN_BOTTOM, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::MARGIN_LEFT, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::MARGIN_RIGHT, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::MARGIN_TOP, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::PADDING, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::PADDING_BOTTOM, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::PADDING_LEFT, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::PADDING_RIGHT, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::PADDING_TOP, CssValue::Keyword("0".to_string()));
    root_layer.set_property(properties::TEXT_ALIGN, CssValue::Keyword("left".to_string()));
    root_layer.set_property(properties::TEXT_DECORATION, CssValue::Keyword("none".to_string()));
    root_layer.set_property(properties::TEXT_TRANSFORM, CssValue::Keyword("none".to_string()));
    root_layer.set_property(properties::WIDTH, CssValue::Keyword("100%".to_string()));
    root_layer.set_property(properties::Z_INDEX, CssValue::Keyword("0".to_string()));

    root_layer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_layer() {
        let mut arena = LayerArena::new();
        let layer_id = arena.new_layer(ROOT_LAYER_ID);
        assert_eq!(layer_id, 1);
        assert_eq!(arena.get_layer(layer_id).unwrap().get_parent_layer_id(), ROOT_LAYER_ID);
    }

    #[test]
    fn test_get_layer() {
        let mut arena = LayerArena::new();
        let layer_id = arena.new_layer(ROOT_LAYER_ID);
        assert_eq!(arena.get_layer(layer_id).unwrap().get_layer_id(), layer_id);
    }

    #[test]
    fn test_get_layer_mut() {
        let mut arena = LayerArena::new();
        let layer_id = arena.new_layer(ROOT_LAYER_ID);
        let layer = arena.get_layer_mut(layer_id).unwrap();
        layer.set_property(properties::BACKGROUND, CssValue::Keyword("red".to_string()));
        assert_eq!(arena.get_layer(layer_id).unwrap().get_property(properties::BACKGROUND), Some(&CssValue::Keyword("red".to_string())));
    }

    #[test]
    fn test_get_root_layer() {
        let arena = LayerArena::new();
        let root_layer = arena.get_root_layer();
        assert_eq!(root_layer.get_layer_id(), ROOT_LAYER_ID);
    }

    #[test]
    fn test_new() {
        let arena = LayerArena::new();
        assert_eq!(arena.get_root_layer().get_layer_id(), ROOT_LAYER_ID);
    }
}