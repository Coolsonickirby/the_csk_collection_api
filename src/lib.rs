#![allow(arithmetic_overflow)]

use std::collections::HashMap;

mod externed {
    extern "C" {
        pub fn play_bgm(ui_bgm_hash: u64);
        pub fn get_color_from_entry_id(entry_id: u32) -> u32;
        pub fn change_entry_chara_ui(entry_id: u32, ui_chara_hash: u64, color_slot: u8);
        pub fn get_ui_chara_from_entry_id(entry_id: u32) -> u64;
        pub fn get_victor_color() -> u8;
        pub fn allow_ui_chara_hash_online(ui_chara_hash: u64);
        pub fn disable_ui_chara_hash_online(ui_chara_hash: u64);
        pub fn is_online() -> bool;
    }
    extern "Rust" {
        pub fn add_chara_db_entry_info(chara_db_entry_info: crate::CharacterDatabaseEntry);
        pub fn add_chara_layout_db_entry_info(chara_db_entry_info: crate::CharacterLayoutDatabaseEntry);
        pub fn csk_collection_version() -> crate::Version;
    }
}

pub fn play_bgm(ui_bgm_hash: u64) {
    unsafe {
        externed::play_bgm(ui_bgm_hash);
    }
}

pub fn get_color_from_entry_id(entry_id: u32) -> u32 {
    unsafe {
        externed::get_color_from_entry_id(entry_id)
    }
}

pub fn change_entry_chara_ui(entry_id: u32, ui_chara_hash: u64, color_slot: u8) {
    unsafe {
        externed::change_entry_chara_ui(entry_id, ui_chara_hash, color_slot);
    }
}

pub fn get_ui_chara_from_entry_id(entry_id: u32) -> u64 {
    unsafe { externed::get_ui_chara_from_entry_id(entry_id) }
}

pub fn get_victor_color() {
    unsafe {
        externed::get_victor_color();
    }
}

pub fn allow_ui_chara_hash_online(ui_chara_hash: u64) {
    unsafe {
        externed::allow_ui_chara_hash_online(ui_chara_hash);
    }
}

pub fn disable_ui_chara_hash_online(ui_chara_hash: u64) {
    unsafe {
        externed::disable_ui_chara_hash_online(ui_chara_hash);
    }
}

pub fn add_chara_db_entry_info(chara_db_entry_info: crate::CharacterDatabaseEntry) {
    unsafe {
        externed::add_chara_db_entry_info(chara_db_entry_info);
    }
}

pub fn add_chara_layout_db_entry_info(chara_layout_db_entry_info: crate::CharacterLayoutDatabaseEntry) {
    unsafe {
        externed::add_chara_layout_db_entry_info(chara_layout_db_entry_info);
    }
}

pub fn is_online() -> bool {
    unsafe { externed::is_online() }
}

pub fn get_plugin_version() -> Version {
    unsafe {
        externed::csk_collection_version()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

macro_rules! create_enum {
    ($field_name:ident: $field_type:ty) => {
        #[repr(C)]
        #[derive(Debug, Clone)]
        pub enum $field_name {
            Overwrite($field_type),
            Optional(Option<$field_type>),
        }

        impl Default for $field_name {
            fn default() -> Self {
                $field_name::Optional(None)
            }
        }
    };
}

create_enum!(StringType: String);
create_enum!(Hash40Type: u64);
create_enum!(ShortType: i16);
create_enum!(IntType: i32);
create_enum!(FloatType: f32);
create_enum!(BoolType: bool);
create_enum!(SignedByteType: i8);
create_enum!(UnsignedByteType: u8);
create_enum!(Hash40Map: HashMap<u64, Hash40Type>);
create_enum!(UnsignedByteMap: HashMap<u64, UnsignedByteType>);

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct CharacterDatabaseEntry {
    pub ui_chara_id: u64,
    pub clone_from_ui_chara_id: Option<u64>,
    pub name_id: StringType,
    pub fighter_kind: Hash40Type,
    pub fighter_kind_corps: Hash40Type,
    pub ui_series_id: Hash40Type,
    pub fighter_type: Hash40Type,
    pub alt_chara_id: Hash40Type,
    pub exhibit_year: ShortType,
    pub exhibit_day_order: IntType,
    pub ext_skill_page_num: SignedByteType,
    pub is_img_ext_skill_page0: BoolType,
    pub is_img_ext_skill_page1: BoolType,
    pub is_img_ext_skill_page2: BoolType,
    pub skill_list_order: SignedByteType,
    pub disp_order: SignedByteType,
    pub save_no: SignedByteType,
    pub chara_count: SignedByteType,
    pub can_select: BoolType,
    pub is_usable_soundtest: BoolType,
    pub is_called_pokemon: BoolType,
    pub is_mii: BoolType,
    pub is_boss: BoolType,
    pub is_hidden_boss: BoolType,
    pub is_dlc: BoolType,
    pub is_patch: BoolType,
    pub is_plural_message: BoolType,
    pub is_plural_narration: BoolType,
    pub is_article: BoolType,
    pub extra_flags: IntType,
    pub unk_0x112b7bb52a: BoolType,
    pub result_pf0: BoolType,
    pub result_pf1: BoolType,
    pub result_pf2: BoolType,
    pub color_num: UnsignedByteType,
    pub extra_index_maps: UnsignedByteMap, // this is going to hold the other three
    // pub cXX_index: UnsignedByteMap,
    // pub nXX_index: UnsignedByteMap,
    // pub cXX_group: UnsignedByteMap,
    pub extra_hash_maps: Hash40Map, // this is going to hold the bottom two
    // pub characall_label: Hash40Map,
    // pub characall_label_article: Hash40Map,
    pub shop_item_tag: Hash40Type,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct CharacterLayoutDatabaseEntry {
    pub ui_layout_id: u64,
    pub clone_from_ui_layout_id: Option<u64>,
    pub ui_chara_id: Hash40Type,
    pub chara_color: UnsignedByteType,
    pub eye_0_flash_count: UnsignedByteType,
    pub eye_0_flash0_pos_x: FloatType,
    pub eye_0_flash0_pos_y: FloatType,
    pub eye_0_flash1_pos_x: FloatType,
    pub eye_0_flash1_pos_y: FloatType,
    pub eye_0_flash2_pos_x: FloatType,
    pub eye_0_flash2_pos_y: FloatType,
    pub eye_0_flash3_pos_x: FloatType,
    pub eye_0_flash3_pos_y: FloatType,
    pub eye_0_flash4_pos_x: FloatType,
    pub eye_0_flash4_pos_y: FloatType,
    pub eye_1_flash_count: UnsignedByteType,
    pub eye_1_flash0_pos_x: FloatType,
    pub eye_1_flash0_pos_y: FloatType,
    pub eye_1_flash1_pos_x: FloatType,
    pub eye_1_flash1_pos_y: FloatType,
    pub eye_1_flash2_pos_x: FloatType,
    pub eye_1_flash2_pos_y: FloatType,
    pub eye_1_flash3_pos_x: FloatType,
    pub eye_1_flash3_pos_y: FloatType,
    pub eye_1_flash4_pos_x: FloatType,
    pub eye_1_flash4_pos_y: FloatType,
    pub eye_2_flash_count: UnsignedByteType,
    pub eye_2_flash0_pos_x: FloatType,
    pub eye_2_flash0_pos_y: FloatType,
    pub eye_2_flash1_pos_x: FloatType,
    pub eye_2_flash1_pos_y: FloatType,
    pub eye_2_flash2_pos_x: FloatType,
    pub eye_2_flash2_pos_y: FloatType,
    pub eye_2_flash3_pos_x: FloatType,
    pub eye_2_flash3_pos_y: FloatType,
    pub eye_2_flash4_pos_x: FloatType,
    pub eye_2_flash4_pos_y: FloatType,
    pub eye_flash_info_pos_x: FloatType,
    pub eye_flash_info_pos_y: FloatType,
    pub spirits_eye_visible: BoolType,
    pub chara_1_offset_x: FloatType,
    pub chara_1_offset_y: FloatType,
    pub chara_1_scale: FloatType,
    pub chara_1_1_offset_x: FloatType,
    pub chara_1_1_offset_y: FloatType,
    pub chara_1_1_scale: FloatType,
    pub chara_1_2_offset_x: FloatType,
    pub chara_1_2_offset_y: FloatType,
    pub chara_1_2_scale: FloatType,
    pub chara_1_3_offset_x: FloatType,
    pub chara_1_3_offset_y: FloatType,
    pub chara_1_3_scale: FloatType,
    pub chara_1_4_offset_x: FloatType,
    pub chara_1_4_offset_y: FloatType,
    pub chara_1_4_scale: FloatType,
    pub chara_1_5_offset_x: FloatType,
    pub chara_1_5_offset_y: FloatType,
    pub chara_1_5_scale: FloatType,
    pub chara_3_0_offset_x: FloatType,
    pub chara_3_0_offset_y: FloatType,
    pub chara_3_0_scale: FloatType,
    pub chara_3_1_offset_x: FloatType,
    pub chara_3_1_offset_y: FloatType,
    pub chara_3_1_scale: FloatType,
    pub chara_3_2_offset_x: FloatType,
    pub chara_3_2_offset_y: FloatType,
    pub chara_3_2_scale: FloatType,
    pub chara_3_3_offset_x: FloatType,
    pub chara_3_3_offset_y: FloatType,
    pub chara_3_3_scale: FloatType,
    pub chara_3_4_offset_x: FloatType,
    pub chara_3_4_offset_y: FloatType,
    pub chara_3_4_scale: FloatType,
    pub chara_3_5_offset_x: FloatType,
    pub chara_3_5_offset_y: FloatType,
    pub chara_3_5_scale: FloatType,
    pub chara_3_6_offset_x: FloatType,
    pub chara_3_6_offset_y: FloatType,
    pub chara_3_6_scale: FloatType,
    pub chara_3_7_offset_x: FloatType,
    pub chara_3_7_offset_y: FloatType,
    pub chara_3_7_scale: FloatType,
    pub chara_5_offset_x: FloatType,
    pub chara_5_offset_y: FloatType,
    pub chara_5_scale: FloatType,
    pub chara_select_icon_list_offset_x: FloatType,
    pub chara_select_icon_list_offset_y: FloatType,
    pub chara_select_icon_list_scale: FloatType,
    pub chara_7_0_offset_x: FloatType,
    pub chara_7_0_offset_y: FloatType,
    pub chara_7_0_scale: FloatType,
    pub chara_7_1_offset_x: FloatType,
    pub chara_7_1_offset_y: FloatType,
    pub chara_7_1_scale: FloatType,
    pub chara_0_offset_x: FloatType,
    pub chara_0_offset_y: FloatType,
    pub chara_0_scale: FloatType,
}

// #[derive(Debug, Copy, Clone)]
// pub struct NarrationCharacallEntry {
//     pub nus3bank_path: u64,
//     pub tonelabel_path: u64,
//     pub nus3audio_path: u64,
//     pub unk_1: u64,
//     pub unk_2: u64,
// }