use std::sync::{Arc, Mutex};
use crate::arena::{LayerArena, LayerId, ROOT_LAYER_ID};
use crate::values::CssValue;

/// A stylelist is nothing more than the arena and a layer id. This layer id is where
/// new properties gets added, while current properties are found in the parent layers
/// of this layer.
pub(crate) struct CssStyleList {
    arena: Arc<Mutex<LayerArena>>,
    layer_id: LayerId
}

impl CssStyleList {
    /// Create a root layer
    pub(crate) fn root(arena: Arc<Mutex<LayerArena>>) -> Self {
        Self {
            arena,
            layer_id: ROOT_LAYER_ID
        }
    }

    /// Create a new stylelist based on the parent stylelist. It will contain a new layer with no
    /// properties.
    pub(crate) fn new(parent: &CssStyleList) -> Self {
        let mut binding = parent.arena.lock().unwrap();
        let layer_id = binding.new_layer(parent.layer_id);
        let parent_layer = binding.get_layer_mut(layer_id).unwrap();

        Self {
            arena: parent.arena.clone(),
            layer_id: parent_layer.layer_id
        }
    }

    /// Sets the given property onto the current stylelists layer
    pub(crate) fn set_property(&mut self, property_name: &'static str, value: CssValue) {
        let mut arena = self.arena.lock().unwrap();
        let layer = arena.get_layer_mut(self.layer_id).unwrap();

        layer.set_property(property_name, value);
    }

    /// Retrieve a property. If not found in the current layer, it will recursively search the
    /// parent layers until it finds the property or reaches the root layer. When not found, it will
    /// return None.
    pub(crate) fn get_property(&self, property_name: &'static str) -> Option<CssValue> {
        let arena = self.arena.lock().unwrap();

        let mut cur_layer_id = self.layer_id;

        loop {
            let layer = arena.get_layer(cur_layer_id).unwrap();

            match layer.get_property(property_name) {
                Some(value) => return Some(value.clone()),
                None => {}
            }

            // Break the loop when we reached the root layer
            if layer.parent_layer_id == cur_layer_id {
                break;
            }

            // Move to the parent layer
            cur_layer_id = layer.parent_layer_id;
        }

        None
    }

    pub(crate) fn layers(&self) -> Vec<LayerId> {
        let arena = self.arena.lock().unwrap();
        let mut cur_layer_id = self.layer_id;
        let mut layers = vec![cur_layer_id];

        loop {
            let layer = arena.get_layer(cur_layer_id).unwrap();
            if layer.parent_layer_id == cur_layer_id {
                break;
            }

            cur_layer_id = layer.parent_layer_id;
            layers.push(cur_layer_id);
        }

        layers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use crate::properties;

    #[test]
    fn test_new_stylelist() {
        let arena = Arc::new(Mutex::new(LayerArena::new()));

        let stylelist = CssStyleList::root(arena.clone());
        assert_eq!(stylelist.layer_id, ROOT_LAYER_ID);
    }

    #[test]
    fn test_set_property() {
        let arena = Arc::new(Mutex::new(LayerArena::new()));
        let mut stylelist = CssStyleList::root(arena.clone());
        stylelist.set_property(properties::BACKGROUND, CssValue::Keyword("red".to_string()));

        let arena = arena.lock().unwrap();
        let layer = arena.get_layer(stylelist.layer_id).unwrap();
        assert_eq!(layer.get_property(properties::BACKGROUND), Some(&CssValue::Keyword("red".to_string())));
    }

    #[test]
    fn test_get_property() {
        let arena = Arc::new(Mutex::new(LayerArena::new()));
        let mut stylelist = CssStyleList::root(arena.clone());
        stylelist.set_property(properties::BACKGROUND, CssValue::Keyword("red".to_string()));

        let value = stylelist.get_property(properties::BACKGROUND);
        assert_eq!(value, Some(&CssValue::Keyword("red".to_string())));
    }
}