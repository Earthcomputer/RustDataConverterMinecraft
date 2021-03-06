use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block, rename_item, rename_poi};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2209;

fn renamer(name: &str) -> Option<String> {
    if name == "minecraft:bee_hive" {
        Some("minecraft:beehive".to_owned())
    } else {
        None
    }
}

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block(types, VERSION, renamer);
    rename_item(types, VERSION, renamer);
    rename_poi(types, VERSION, renamer);
}
