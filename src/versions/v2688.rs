use rust_dataconverter_engine::{DataWalkerMapListPaths, DataWalkerMapTypePaths, Types};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2688;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:glow_squid", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:glow_item_frame", DataWalkerMapTypePaths::new(types.item_stack, "Item"));
}
