use rust_dataconverter_engine::{DataWalkerMapListPaths, Types};
use crate::helpers::rename::{rename_entity, rename_item, simple_rename};
use crate::MinecraftTypesMut;

const VERSION: u32 = 2509;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_item(types, VERSION, simple_rename("minecraft:zombie_pigman_spawn_egg", "minecraft:zombified_piglin_spawn_egg"));
    rename_entity(types, VERSION, simple_rename("minecraft:zombie_pigman", "minecraft:zombified_piglin"));
    types.entity.borrow_mut().add_walker_for_id(VERSION, "minecraft:zombified_piglin", DataWalkerMapListPaths::new_multi(types.item_stack, vec!["ArmorItems".to_owned(), "HandItems".to_owned()]));
}
