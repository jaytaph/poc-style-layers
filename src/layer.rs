use std::collections::HashMap;
use crate::arena::LayerId;
use crate::values::CssValue;

pub(crate) struct CssPropertyListLayer {
    // pub(crate) layer_arena: Arc<Mutex<LayerArena>>,
    pub(crate) layer_id: LayerId,
    pub(crate) parent_layer_id: LayerId,
    properties: HashMap<&'static str, CssValue>,
}

impl CssPropertyListLayer {
    pub(crate) fn new(layer_id: LayerId, parent_layer_id: LayerId) -> Self {
        Self {
            layer_id,
            parent_layer_id,
            properties: HashMap::new(),
        }
    }

    pub(crate) fn set_property(&mut self, property_name: &'static str, value: CssValue) {
        self.properties.insert(property_name, value);
    }

    pub(crate) fn get_property(&self, property_name: &'static str) -> Option<&CssValue> {
        self.properties.get(property_name)
    }
}
