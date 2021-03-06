use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_block, rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1490;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_block::<T>(types, VERSION, simple_rename("minecraft:melon_block", "minecraft:melon"));
    rename_item::<T>(types, VERSION, |name| {
        match name {
            "minecraft:melon_block" => Some("minecraft:melon".to_owned()),
            "minecraft:melon" => Some("minecraft:melon_slice".to_owned()),
            "minecraft:speckled_melon" => Some("minecraft:glistering_melon_slice".to_owned()),
            _ => None
        }
    });
}
