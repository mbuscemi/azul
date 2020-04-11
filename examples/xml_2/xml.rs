#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate azul;

use azul::prelude::*;
use std::time::Duration;

macro_rules! XML_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/xml/ui.xml")) }
macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/../../examples/xml/xml.css")) }

struct DataModel { }

impl Layout for DataModel {
    fn layout(&self, _: LayoutInfo<DataModel>) -> Dom<DataModel> {
        DomXml::from_file(XML_PATH!(), &mut XmlComponentMap::default()).into()
    }
}

fn main() {
    let mut app = App::new(DataModel { }, AppConfig::default()).unwrap();
    app.run(WindowCreateOptions::default());
}
