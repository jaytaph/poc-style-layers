use crate::stylelist::CssStyleList;

mod properties;
mod values;
mod stylelist;

fn main() {
    let root_sheet = CssStyleList::root();
    println!("Root layer");
    println!("Root : {:?}", root_sheet.get_property(properties::BACKGROUND));
    println!("Root : {:?}", root_sheet.get_property(properties::WIDTH));

    println!("Next1 layer");
    let mut next_sheet_1 = root_sheet.create();
    next_sheet_1.set_property(properties::BACKGROUND, values::CssValue::Keyword("red".to_string()));
    println!("Next1: {:?}", next_sheet_1.get_property(properties::BACKGROUND));
    println!("Next1: {:?}", next_sheet_1.get_property(properties::WIDTH));

    println!("Next2 layer");
    let mut next_sheet_2 = next_sheet_1.create();
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
    let mut next_sheet_3 = next_sheet_1.create();
    next_sheet_3.set_property(properties::BACKGROUND, values::CssValue::Keyword("rebeccapurple".to_string()));
    println!("Next3: {:?}", next_sheet_3.get_property(properties::BACKGROUND));
    println!("Next3: {:?}", next_sheet_3.get_property(properties::WIDTH));
}
