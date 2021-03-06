use rust_dataconverter_engine::Types;
use crate::helpers::rename::{rename_entity, rename_item};
use crate::MinecraftTypesMut;

const VERSION: u32 = 1486;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    rename_entity::<T>(types, VERSION, |name| {
        match name {
            "minecraft:salmon_mob" => Some("minecraft:salmon".to_owned()),
            "minecraft:cod_mob" => Some("minecraft:cod".to_owned()),
            _ => None
        }
    });
    rename_item::<T>(types, VERSION, |name| {
        match name {
            "minecraft:salmon_mob_spawn_egg" => Some("minecraft:salmon_spawn_egg".to_owned()),
            "minecraft:cod_mob_spawn_egg" => Some("minecraft:cod_spawn_egg".to_owned()),
            _ => None
        }
    });

    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:salmon_mob", "minecraft:salmon");
    types.entity.borrow_mut().copy_walkers(VERSION, "minecraft:cod_mob", "minecraft:cod");
}
