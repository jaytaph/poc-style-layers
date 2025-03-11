use std::sync::{Arc, Mutex};
use crate::stylelist::CssStyleList;

mod properties;
mod arena;
mod values;
mod stylelist;
mod layer;

fn main() {
    let arena = Arc::new(Mutex::new(arena::LayerArena::new()));

    let root_sheet = CssStyleList::root(arena.clone());
    println!("Root layer");
    println!("Root : {:?}", root_sheet.get_property(properties::BACKGROUND));
    println!("Root : {:?}", root_sheet.get_property(properties::WIDTH));

    println!("Next1 layer");
    let mut next_sheet_1 = CssStyleList::new(&root_sheet);
    next_sheet_1.set_property(properties::BACKGROUND, values::CssValue::Keyword("red".to_string()));
    println!("Next1: {:?}", next_sheet_1.get_property(properties::BACKGROUND));
    println!("Next1: {:?}", next_sheet_1.get_property(properties::WIDTH));

    println!("Next2 layer");
    let mut next_sheet_2 = CssStyleList::new(&next_sheet_1);
    next_sheet_2.set_property(properties::BACKGROUND, values::CssValue::Keyword("green".to_string()));
    println!("Next2: {:?}", next_sheet_2.get_property(properties::BACKGROUND));
    println!("Next2: {:?}", next_sheet_2.get_property(properties::WIDTH));

    next_sheet_2.set_property(properties::BACKGROUND, values::CssValue::Keyword("green".to_string()));
    println!("Next2: {:?}", next_sheet_2.get_property(properties::WIDTH));
    println!("Next1: {:?}", next_sheet_1.get_property(properties::WIDTH));

    println!("All layers");
    println!("Root : {:?}", root_sheet.get_property(properties::BACKGROUND));
    println!("Next1: {:?}", next_sheet_1.get_property(properties::BACKGROUND));
    println!("Next2: {:?}", next_sheet_2.get_property(properties::BACKGROUND));


    println!("Next3 layer");
    let mut next_sheet_3 = CssStyleList::new(&next_sheet_1);
    next_sheet_3.set_property(properties::BACKGROUND, values::CssValue::Keyword("rebeccapurple".to_string()));
    println!("Next3: {:?}", next_sheet_3.get_property(properties::BACKGROUND));
    println!("Next3: {:?}", next_sheet_3.get_property(properties::WIDTH));

    println!("Layers");
    println!("{:?}", root_sheet.layers());
    println!("{:?}", next_sheet_1.layers());
    println!("{:?}", next_sheet_2.layers());
    println!("{:?}", next_sheet_3.layers());
}
