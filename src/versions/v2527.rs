use rust_dataconverter_engine::{data_converter_func, ListType, MapType, ObjectRef, ObjectType, Types};
use crate::helpers::bit_storage::ceil_log2;
use crate::MinecraftTypesMut;

const VERSION: u32 = 2527;

pub(crate) fn register<T: Types + ?Sized>(types: &MinecraftTypesMut<T>) {
    types.chunk.borrow_mut().add_structure_converter(VERSION, data_converter_func::<T::Map, _>(|data, _from_version, _to_version| {
        if let Some(level) = data.get_map_mut("Level") {
            if let Some(sections) = level.get_list_mut("Sections") {
                for section in sections.iter_mut() {
                    if let Some(section) = section.as_map_mut() {
                        let section: &mut T::Map = section;
                        if let Some(palette) = section.get_map("Palette") {
                            let bits = 4.max(ceil_log2(palette.size() as u32));
                            if bits.is_power_of_two() {
                                // fits perfectly
                                continue;
                            }
                            if let Some(ObjectRef::LongArray(states)) = section.get("BlockStates").map(|o| o.as_ref()) {
                                let new_states = add_padding(4096, bits as usize, states);
                                section.set("BlockStates", T::Object::create_long_array(new_states));
                            }
                        }
                    }
                }
            }

            if let Some(heightmaps) = level.get_map_mut("Heightmaps") {
                for key in heightmaps.keys().cloned().collect::<Vec<_>>() {
                    if let ObjectRef::LongArray(old) = heightmaps.get(&key).unwrap().as_ref() {
                        let new = add_padding(256, 9, old);
                        heightmaps.set(key, T::Object::create_long_array(new));
                    }
                }
            }
        }
    }));
}

// Assumes that bits is *not* a power of 2!
fn add_padding(size: usize, bits: usize, old: &[i64]) -> Vec<i64> {
    let old_len = old.len();
    if old_len == 0 {
        return Vec::new();
    }

    let mask = (1i64 << bits) - 1;
    let values_per_long = 64 / bits;
    let new_len = (size + values_per_long - 1) / values_per_long;
    let mut padded = vec![0i64; new_len];
    let mut new_word_index = 0;
    let mut used_bits = 0;
    let mut new_word = 0;
    let mut prev_old_word_index = 0;
    let mut old_word = old[0];
    let mut old_next_word = if old_len > 1 { old[1] } else { 0 };

    for index in 0..size {
        let old_bit_index = index * bits;
        let old_word_index = old_bit_index >> 6;
        let old_end_word_index = ((index + 1) * bits - 1) >> 6;
        let old_index_in_word = old_bit_index ^ (old_word_index << 6);
        if old_word_index != prev_old_word_index {
            old_word = old_next_word;
            old_next_word = if old_word_index + 1 < old_len { old[old_word_index + 1] } else { 0 };
            prev_old_word_index = old_word_index;
        }

        let value = if old_word_index == old_end_word_index {
            ((old_word as u64) >> old_index_in_word) as i64 & mask
        } else {
            let first_bits = 64 - old_index_in_word;
            (((old_word as u64) >> old_index_in_word) as i64 | old_next_word << first_bits) & mask
        };

        let next_used_bits = used_bits + bits;
        if next_used_bits >= 64 {
            padded[new_word_index] = new_word;
            new_word_index += 1;
            new_word = value;
            used_bits = bits;
        } else {
            new_word |= value << used_bits;
            used_bits = next_used_bits;
        }
    }

    if new_word != 0 {
        padded[new_word_index] = new_word;
    }

    padded
}
