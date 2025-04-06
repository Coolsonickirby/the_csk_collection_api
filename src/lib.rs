#![allow(arithmetic_overflow)]
extern crate smash_sli;

use std::{collections::HashMap, ffi::CString};
use std::io::{BufRead, Error, Read, Seek, Write};
use binrw::BinWriterExt;


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
        pub fn csk_collection_version() -> *const crate::Version;
        pub fn add_narration_characall_entry(string_ptr: *mut i8) -> bool;
        pub fn set_fighter_jingle(chara_id: u64, string_ptr: *mut i8);
        pub fn load_ui_file(ui_path: u64);
    }
    extern "Rust" {
        pub fn add_chara_db_entry_info(chara_db_entry_info: &crate::CharacterDatabaseEntry);
        pub fn add_chara_layout_db_entry_info(chara_db_entry_info: &crate::CharacterLayoutDatabaseEntry);
        pub fn add_series_db_entry_info(series_db_entry_info: &crate::SeriesDatabaseEntry);
        pub fn add_bgm_db_entry_info(bgm_db_entry_info: &crate::BgmDatabaseRootEntry);
        pub fn add_stream_set_entry_info(stream_set_entry_info: &crate::StreamSetEntry);
        pub fn add_assigned_info_entry_info(assigned_info_entry_info: &crate::AssignedInfoEntry);
        pub fn add_stream_property_entry_info(stream_property_entry_info: &crate::StreamPropertyEntry);
        pub fn add_new_sli_entry(entry: &smash_sli::SliEntry);
        pub fn add_new_bgm_property_entry(entry: &smash_bgm_property::BgmPropertyEntry);
        pub fn add_tracks_to_playlist(playlist: u64, tracks: &Vec<crate::BgmPlaylistEntry>);
        pub fn add_stage_db_entry(stage_entry: &crate::StageDatabaseEntry);
        pub fn add_ui_stage_db_resources_entry(stage_place_id: u64, ui_stage_id: u64, stage_data: &crate::UiStageData);
        pub fn add_gametitle_db_entry_info(gametitle_db_entry_info: &crate::GametitleDatabaseEntry);
        pub fn add_tips_db_entry_info(tips_db_entry_info: &crate::TipsDatabaseEntry);
        pub fn add_amiibo_db_entry_info(amiibo_db_entry_info: &crate::AmiiboDatabaseEntry);
        pub fn add_mii_body_db_entry_info(mii_body_db_entry_info: &crate::MiiBodyDatabaseEntry);
        pub fn add_mii_hat_db_entry_info(mii_hat_db_entry_info: &crate::MiiHatDatabaseEntry);
    }
}


pub fn play_bgm(ui_bgm_hash: u64) {
    unsafe {
        externed::play_bgm(ui_bgm_hash);
    }
}



pub fn get_color_from_entry_id(entry_id: u32) -> u32 {
    unsafe { externed::get_color_from_entry_id(entry_id) }
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
        externed::add_chara_db_entry_info(&chara_db_entry_info);
    }
}


pub fn add_chara_layout_db_entry_info(
    chara_layout_db_entry_info: crate::CharacterLayoutDatabaseEntry,
) {
    unsafe {
        externed::add_chara_layout_db_entry_info(&chara_layout_db_entry_info);
    }
}


pub fn add_series_db_entry_info(series_db_entry_info: crate::SeriesDatabaseEntry) {
    unsafe {
        externed::add_series_db_entry_info(&series_db_entry_info);
    }
}

pub fn add_bgm_db_entry_info(bgm_db_entry_info: &crate::BgmDatabaseRootEntry) {
    unsafe { externed::add_bgm_db_entry_info(bgm_db_entry_info); }
}
pub fn add_stream_set_entry_info(stream_set_entry_info: &crate::StreamSetEntry) {
    unsafe { externed::add_stream_set_entry_info(stream_set_entry_info); }
}
pub fn add_assigned_info_entry_info(assigned_info_entry_info: &crate::AssignedInfoEntry) {
    unsafe { externed::add_assigned_info_entry_info(assigned_info_entry_info); }
}
pub fn add_stream_property_entry_info(stream_property_entry_info: &crate::StreamPropertyEntry) {
    unsafe { externed::add_stream_property_entry_info(stream_property_entry_info); }
}

pub fn add_gametitle_db_entry_info(gametitle_db_entry_info: &crate::GametitleDatabaseEntry) {
    unsafe { externed::add_gametitle_db_entry_info(gametitle_db_entry_info); }
}
pub fn add_tips_db_entry_info(tips_db_entry_info: &crate::TipsDatabaseEntry) {
    unsafe { externed::add_tips_db_entry_info(tips_db_entry_info); }
}
pub fn add_amiibo_db_entry_info(amiibo_db_entry_info: &crate::AmiiboDatabaseEntry) {
    unsafe { externed::add_amiibo_db_entry_info(amiibo_db_entry_info); }
}
pub fn add_mii_body_db_entry_info(mii_body_db_entry_info: &crate::MiiBodyDatabaseEntry) {
    unsafe { externed::add_mii_body_db_entry_info(mii_body_db_entry_info); }
}
pub fn add_mii_hat_db_entry_info(mii_hat_db_entry_info: &crate::MiiHatDatabaseEntry) {
    unsafe { externed::add_mii_hat_db_entry_info(mii_hat_db_entry_info); }
}   

pub fn add_stage_db_entry(stage_entry: &crate::StageDatabaseEntry) {
    unsafe { externed::add_stage_db_entry(stage_entry); }
}

pub fn add_ui_stage_db_resources_entry(stage_place_id: u64, ui_stage_id: u64, stage_data: &crate::UiStageData){
    unsafe { externed::add_ui_stage_db_resources_entry(stage_place_id, ui_stage_id, stage_data); }
}


pub fn add_narration_characall_entry(entry: &str) -> bool {
    unsafe {
        let ptr = std::ffi::CString::new(entry)
            .expect(&format!("Failed converting {} to CString!", entry))
            .into_raw();
        externed::add_narration_characall_entry(ptr as _)
    }
}


pub fn add_new_sli_entry(entry: &smash_sli::SliEntry) {
    unsafe {
        externed::add_new_sli_entry(entry);
    }
}

pub fn add_new_bgm_property_entry(entry: &smash_bgm_property::BgmPropertyEntry) {
    unsafe {
        externed::add_new_bgm_property_entry(entry);
    }
}

pub fn set_fighter_jingle(chara_id: u64, entry: &str){
    unsafe {
        let ptr = std::ffi::CString::new(entry)
        .expect(&format!("Failed converting {} to CString!", entry))
        .into_raw();
    externed::set_fighter_jingle(chara_id, ptr as _)
}
}

pub fn add_tracks_to_playlist(playlist: u64, tracks: &Vec<crate::BgmPlaylistEntry>) {
    unsafe {
        externed::add_tracks_to_playlist(playlist, tracks);
    }
}

pub fn load_ui_file(ui_path: u64) {
    unsafe { externed::load_ui_file(ui_path); }
}


pub fn is_online() -> bool {
    unsafe { externed::is_online() }
}


pub fn get_plugin_version() -> Version {
    unsafe { *externed::csk_collection_version() }
}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CStrCSK {
    pub ptr: *mut i8,
}

impl CStrCSK {
    pub fn empty() -> Self {
        CStrCSK { ptr: std::ptr::null_mut() }
    }

    pub fn new(s: &str) -> Self {
        let entry = CStrCSK {
            ptr: std::ptr::null_mut(),
        };
        entry.set(s)
    }

    pub fn set(mut self, s: &str) -> Self {
        unsafe {
            if !self.ptr.is_null() {
                drop(CString::from_raw(self.ptr as _));
            }
            self.ptr = CString::new(s).unwrap().into_raw() as _;
            self
        }
    }

    // Consumes itself after getting the string
    pub fn get(self) -> Option<String> {
        unsafe {
            if self.ptr.is_null() {
                return None;
            }
            let from_raw = CString::from_raw(self.ptr as _);
            match from_raw.to_str() {
                Ok(res) => Some(res.to_string()),
                Err(_) => None,
            }
        }
    }
}

impl Default for CStrCSK {
    fn default() -> Self {
        CStrCSK::empty()
    }
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

create_enum!(StringType: CStrCSK);
create_enum!(Hash40Type: u64);
create_enum!(ShortType: i16);
create_enum!(UnsignedShortType: u16);
create_enum!(IntType: i32);
create_enum!(UnsignedIntType: u32);
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
    pub has_multiple_face: BoolType,
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

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct SeriesDatabaseEntry {
    pub ui_series_id: u64,
    pub clone_from_ui_series_id: Option<u64>,
    pub name_id: StringType,
    pub disp_order: SignedByteType,
    pub disp_order_sound: SignedByteType,
    pub save_no: SignedByteType,
    pub shown_as_series_in_directory: BoolType,
    pub is_dlc: BoolType,
    pub is_patch: BoolType,
    pub dlc_chara_id: Hash40Type,
    pub is_use_amiibo_bg: BoolType,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct BgmDatabaseRootEntry {
    pub ui_bgm_id: u64,
    pub clone_from_ui_bgm_id: Option<u64>,
    pub stream_set_id: Hash40Type,
    pub rarity: Hash40Type,
    pub record_type: Hash40Type,
    pub ui_gametitle_id: Hash40Type,
    pub ui_gametitle_id_1: Hash40Type,
    pub ui_gametitle_id_2: Hash40Type,
    pub ui_gametitle_id_3: Hash40Type,
    pub ui_gametitle_id_4: Hash40Type,
    pub name_id: StringType,
    pub save_no: ShortType,
    pub test_disp_order: ShortType,
    pub menu_value: IntType,
    pub jp_region: BoolType,
    pub other_region: BoolType,
    pub possessed: BoolType,
    pub prize_lottery: BoolType,
    pub shop_price: UnsignedIntType,
    pub count_target: BoolType,
    pub menu_loop: UnsignedByteType,
    pub is_selectable_stage_make: BoolType,
    pub is_selectable_movie_edit: BoolType,
    pub is_selectable_original: BoolType,
    pub is_dlc: BoolType,
    pub is_patch: BoolType,
    pub dlc_ui_chara_id: Hash40Type,
    pub dlc_mii_hat_motif_id: Hash40Type,
    pub dlc_mii_body_motif_id: Hash40Type,
    pub unk_0x0e6b57e593: BoolType
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct StreamSetEntry {
    pub stream_set_id: u64,
    pub clone_from_stream_set_id: Option<u64>,
    pub special_category: Hash40Type,
    pub info0: Hash40Type,
    pub info1: Hash40Type,
    pub info2: Hash40Type,
    pub info3: Hash40Type,
    pub info4: Hash40Type,
    pub info5: Hash40Type,
    pub info6: Hash40Type,
    pub info7: Hash40Type,
    pub info8: Hash40Type,
    pub info9: Hash40Type,
    pub info10: Hash40Type,
    pub info11: Hash40Type,
    pub info12: Hash40Type,
    pub info13: Hash40Type,
    pub info14: Hash40Type,
    pub info15: Hash40Type,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct AssignedInfoEntry {
    pub info_id: u64,
    pub clone_from_info_id: Option<u64>,
    pub stream_id: Hash40Type,
    pub condition: Hash40Type,
    pub condition_process: Hash40Type,
    pub start_frame: IntType,
    pub change_fadein_frame: IntType,
    pub change_start_delay_frame: IntType,
    pub change_fadeout_frame: IntType,
    pub change_stop_delay_frame: IntType,
    pub menu_change_fadein_frame: IntType,
    pub menu_change_start_delay_frame: IntType,
    pub menu_change_fadeout_frame: IntType,
    pub menu_change_stop_delay_frame: IntType,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct StreamPropertyEntry {
    pub stream_id: u64,
    pub clone_from_stream_id: Option<u64>,
    pub data_name0: StringType,
    pub data_name1: StringType,
    pub data_name2: StringType,
    pub data_name3: StringType,
    pub data_name4: StringType,
    pub loop_track: UnsignedByteType,
    pub end_point: StringType,
    pub fadeout_frame: UnsignedShortType,
    pub start_point_suddendeath: StringType,
    pub start_point_transition: StringType,
    pub start_point0: StringType,
    pub start_point1: StringType,
    pub start_point2: StringType,
    pub start_point3: StringType,
    pub start_point4: StringType,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct BgmPlaylistEntry {
    pub ui_bgm_id: u64,
    pub order0: ShortType,
    pub incidence0: UnsignedShortType,
    pub order1: ShortType,
    pub incidence1: UnsignedShortType,
    pub order2: ShortType,
    pub incidence2: UnsignedShortType,
    pub order3: ShortType,
    pub incidence3: UnsignedShortType,
    pub order4: ShortType,
    pub incidence4: UnsignedShortType,
    pub order5: ShortType,
    pub incidence5: UnsignedShortType,
    pub order6: ShortType,
    pub incidence6: UnsignedShortType,
    pub order7: ShortType,
    pub incidence7: UnsignedShortType,
    pub order8: ShortType,
    pub incidence8: UnsignedShortType,
    pub order9: ShortType,
    pub incidence9: UnsignedShortType,
    pub order10: ShortType,
    pub incidence10: UnsignedShortType,
    pub order11: ShortType,
    pub incidence11: UnsignedShortType,
    pub order12: ShortType,
    pub incidence12: UnsignedShortType,
    pub order13: ShortType,
    pub incidence13: UnsignedShortType,
    pub order14: ShortType,
    pub incidence14: UnsignedShortType,
    pub order15: ShortType,
    pub incidence15: UnsignedShortType,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct StageDatabaseEntry {
    pub ui_stage_id: u64,
    pub clone_from_ui_stage_id: Option<u64>,
    pub name_id: StringType,
    pub save_no: ShortType,
    pub ui_series_id: Hash40Type,
    pub can_select: BoolType,
    pub disp_order: SignedByteType,
    pub stage_place_id: Hash40Type,
    pub secret_stage_place_id: Hash40Type,
    pub can_demo: BoolType,
    pub is_8player_stage: BoolType,
    pub is_usable_flag: BoolType,
    pub is_usable_amiibo: BoolType,
    pub secret_command_id: Hash40Type,
    pub secret_command_id_joycon: Hash40Type,
    pub bgm_set_id: Hash40Type,
    pub bgm_setting_no: UnsignedByteType,
    pub bgm_selector: BoolType,
    pub is_dlc: BoolType,
    pub is_patch: BoolType,
    pub dlc_chara_id: Hash40Type,
    pub extra_hash_maps: Hash40Map,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct GametitleDatabaseEntry {
    pub ui_gametitle_id: u64,
    pub clone_from_ui_gametitle_id: Option<u64>,
    pub name_id: StringType,
    pub ui_series_id: Hash40Type,
    pub shown_as_series_in_directory: BoolType,
    pub release: IntType
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct TipsDatabaseEntry {
    pub ui_tips_id: u64,
    pub clone_from_ui_tips_id: Option<u64>,
    pub save_no: UnsignedIntType,
    pub level: Hash40Type,
    pub topic: Hash40Type,
    pub skill_kind: Hash40Type,
    pub ui_tips_unlock_id: Hash40Type,
    pub disp_order: UnsignedIntType,
    pub type_0: Hash40Type,
    pub key_0: Hash40Type,
    pub type_1: Hash40Type,
    pub key_1: Hash40Type,
    pub type_2: Hash40Type,
    pub key_2: Hash40Type,
    pub type_3: Hash40Type,
    pub key_3: Hash40Type,
    pub type_4: Hash40Type,
    pub key_4: Hash40Type,
    pub type_5: Hash40Type,
    pub key_5: Hash40Type,
    pub type_6: Hash40Type,
    pub key_6: Hash40Type,
    pub type_7: Hash40Type,
    pub key_7: Hash40Type,
    pub type_8: Hash40Type,
    pub key_8: Hash40Type,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct AmiiboDatabaseEntry {
    pub ui_amiibo_id: u64,
    pub clone_from_ui_amiibo_id: Option<u64>,
    pub ui_chara_id: Hash40Type,
    pub is_valid: BoolType,
    pub unk_0x13a26bd6a0: BoolType,
    pub nfp_character_id_upper: UnsignedShortType,
    pub nfp_character_id_lower: UnsignedByteType,
    pub enable_unknown_numbering_id: BoolType,
    pub nfp_numbering_id: UnsignedShortType,
    pub default_color: UnsignedByteType
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct MiiBodyDatabaseEntry {
    pub ui_mii_body_id: u64,
    pub clone_from_ui_mii_body_id: Option<u64>,
    pub name_id: StringType,
    pub mii_body_id: StringType,
    pub valid_resource: SignedByteType,
    pub motif_gender: Hash40Type,
    pub motif_id: Hash40Type,
    pub text_id: StringType,
    pub replace_id: StringType,
    pub normal_suit: UnsignedByteType,
    pub dlc_type: Hash40Type,
    pub is_patch: BoolType,
    pub save_no: ShortType,
    pub mii_body_type: Hash40Type,
    pub gender: Hash40Type,
    pub unk_0x18ef467708: BoolType,
    pub prize_lottery: BoolType,
    pub rarity: Hash40Type,
    pub disp_order: IntType,
    pub shop_price: UnsignedIntType,
    pub color_num: IntType,
    pub swing_enabled: BoolType,
    pub shop_item_tag: Hash40Type,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct MiiHatDatabaseEntry {
    pub ui_mii_hat_id: u64,
    pub clone_from_ui_mii_hat_id: Option<u64>,
    pub name_id: StringType,
    pub mii_hat_id: StringType,
    pub valid_resource: SignedByteType,
    pub dlc_type: Hash40Type,
    pub is_patch: BoolType,
    pub save_no: ShortType,
    pub gender: Hash40Type,
    pub motif_gender: Hash40Type,
    pub motif_id: Hash40Type,
    pub text_id: StringType,
    pub unk_0x18ef467708: BoolType,
    pub prize_lottery: BoolType,
    pub rarity: Hash40Type,
    pub disp_order: IntType,
    pub shop_price: UnsignedIntType,
    pub mii_model_type: Hash40Type,
    pub mii_parts_transform: Hash40Type,
    pub unk_0x10b20e051d: BoolType,
    pub shop_item_tag: Hash40Type,
    pub f_cam_pos_x: FloatType,
    pub f_cam_pos_y: FloatType,
    pub f_cam_pos_z: FloatType,
    pub f_cam_rot_x: FloatType,
    pub f_cam_rot_y: FloatType,
    pub f_cam_rot_z: FloatType,
    pub s_cam_pos_x: FloatType,
    pub s_cam_pos_y: FloatType,
    pub s_cam_pos_z: FloatType,
    pub s_cam_rot_x: FloatType,
    pub s_cam_rot_y: FloatType,
    pub s_cam_rot_z: FloatType,
    pub g_cam_pos_x: FloatType,
    pub g_cam_pos_y: FloatType,
    pub g_cam_pos_z: FloatType,
    pub g_cam_rot_x: FloatType,
    pub g_cam_rot_y: FloatType,
    pub g_cam_rot_z: FloatType,
    pub swing_enabled: BoolType,
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct UiStageData {
    pub normal: UiStageResources,
    pub end: UiStageResources,
    pub battle: UiStageResources
}

#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct UiStageResources {
    pub stage_load_group_hash: u64,
    pub effect_load_group_hash: u64,
    pub nus3bank_path_hash: u64,
    pub sqb_path_hash: u64,
    pub nus3audio_path_hash: u64,
    pub tonelabel_path_hash: u64,    
}

pub fn append_entries_to_nus3bank(
    data: &mut [u8],
    source_name: &str,
    new_entries: &Vec<String>,
) -> std::result::Result<Vec<u8>, String> {
    let source_name = source_name.as_bytes();
    let source_name_offset = data
        .windows(source_name.len())
        .position(|window| window == source_name)
        .unwrap();

    let mut cursor = std::io::Cursor::new(data);

    let buf: &mut [u8; 4] = &mut [0; 4];
    cursor.read_exact(buf).unwrap();

    match std::str::from_utf8(buf) {
        Ok(res) => {
            if res != "NUS3" {
                return Err(format!("Your nus3bank file magic does not equal to NUS3! Read: {} = Aborting merging process.", res));
            }
        }
        Err(err) => {
            return Err(format!("Failed reading your nus3bank file magic! Reason: {:?}", err));
        }
    }

    let nus3bank_size = read_u32(&mut cursor);

    let buf: &mut [u8; 8] = &mut [0; 8];
    cursor.read_exact(buf).unwrap();

    match std::str::from_utf8(buf) {
        Ok(res) => {
            if res != "BANKTOC " {
                return Err(format!("Did not read BANKTOC! Your nus3bank file may be malformed. Read: {} = Aborting merging process.", res));
            }
        }
        Err(err) => {
            return Err(format!("Did not find BANKTOC string at expected offset! Your nus3bank file may be malformed. Reason: {:?}", err));
        }
    }

    let toc_size = read_u32(&mut cursor);

    let content_count = read_u32(&mut cursor);

    let mut offset = 0x14 + toc_size;
    let mut tone_offset: u32 = 0;
    let mut tone_size: u32 = 0;
    let mut tone_header_offset: usize = 0;
    for x in 0..content_count {
        let content = read_u32(&mut cursor);
        tone_header_offset = cursor.position() as usize;
        let content_size = read_u32(&mut cursor);

        if content == 1162760020 {
            // TONE
            tone_offset = offset;
            tone_size = content_size;
            break;
        }
        offset += 8 + content_size
    }

    if tone_offset == 0 {
        return Err(format!("Failed getting the tone offset! Aborting merging process."));
    }

    cursor
        .seek(std::io::SeekFrom::Start(tone_offset as u64))
        .unwrap();

    let buf: &mut [u8; 4] = &mut [0; 4];
    cursor.read_exact(buf).unwrap();

    match std::str::from_utf8(buf) {
        Ok(res) => {
            if res != "TONE" {
                return Err(format!("Did not read TONE! Your nus3bank file may be malformed. Read: {} = Aborting merging process.", res));
            }
        }
        Err(err) => {
            return Err(format!("Did not find TONE string at expected offset! Your nus3bank file may be malformed. Reason: {:?}", err));
        }
    }

    let tone_size_check = read_u32(&mut cursor);
    if tone_size_check != tone_size {
        return Err(format!("TONE section size does not match expected tone size! Expected: {} - Read: {} = Aborting merge process.", tone_size, tone_size_check));
    }

    let tone_count = read_u32(&mut cursor);

    cursor
        .seek(std::io::SeekFrom::Start(source_name_offset as u64))
        .unwrap();
    let (source_meta_offset, source_meta_size) = get_sub_meta_offset_and_size(&mut cursor);

    cursor
        .seek(std::io::SeekFrom::Start(source_meta_offset as u64))
        .unwrap();

    cursor
        .seek(std::io::SeekFrom::Start(source_name_offset as u64 - 0xD))
        .unwrap();

    let source_pre_meta_data: &mut [u8; 0xC] = &mut [0; 0xC];
    cursor.read_exact(source_pre_meta_data).unwrap();

    cursor
        .seek(std::io::SeekFrom::Start(
            (tone_offset + 12 + (tone_count - 1) * 8) as u64,
        ))
        .unwrap();
    let last_tone_offset = read_u32(&mut cursor);
    let last_tone_size = read_u32(&mut cursor);

    let mut new_total_tone_size: usize = 0;

    for x in new_entries.iter() {
        let mut tone_size = (source_meta_size + 28 + x.len() as u64 + 1) as usize;
        tone_size += 4 - ((x.len() + 1) % 4);
        new_total_tone_size += tone_size;
    }

    cursor.seek(std::io::SeekFrom::Start(0)).unwrap();
    let n3b_data = cursor.into_inner();

    let output_file: Vec<u8> = Vec::new();
    let mut output_cursor = std::io::Cursor::new(output_file);

    output_cursor.write("NUS3".as_bytes()).unwrap();
    output_cursor
        .write_le(&u32::to_le_bytes(
            nus3bank_size + (new_total_tone_size + (8 * new_entries.len())) as u32,
        ))
        .unwrap();
    output_cursor
        .write(&n3b_data[8..tone_header_offset])
        .unwrap();
    output_cursor
        .write_le(&u32::to_le_bytes(
            tone_size + (new_total_tone_size as u32) + (8 * new_entries.len() as u32),
        ))
        .unwrap();
    output_cursor
        .write(&n3b_data[(tone_header_offset + 4)..(tone_offset + 4) as usize])
        .unwrap();
    output_cursor
        .write_le(&u32::to_le_bytes(
            tone_size + (new_total_tone_size as u32) + (8 * new_entries.len() as u32),
        ))
        .unwrap();
    output_cursor
        .write_le(&u32::to_le_bytes(tone_count + new_entries.len() as u32))
        .unwrap();

    let mut counter = 0;
    while counter < tone_count {
        let current_tone_offset = u32::from_le_bytes(
            n3b_data[(tone_offset + 12 + counter * 8) as usize
                ..(tone_offset + 16 + counter * 8) as usize]
                .try_into()
                .unwrap(),
        );
        output_cursor
            .write_le(&u32::to_le_bytes(
                current_tone_offset + (8 * new_entries.len() as u32),
            ))
            .unwrap();
        output_cursor
            .write(
                &n3b_data[(tone_offset + 16 + counter * 8) as usize
                    ..(tone_offset + 20 + counter * 8) as usize],
            )
            .unwrap();
        counter += 1;
    }

    let mut last_tone_offset_counter =
        last_tone_offset + last_tone_size + (8 * new_entries.len() as u32);
    for x in 0..new_entries.len() {
        let mut tone_size = (source_meta_size + 28 + new_entries[x].len() as u64 + 1) as usize;
        tone_size += 4 - ((new_entries[x].len() + 1) % 4);
        output_cursor
            .write_le(&u32::to_le_bytes(last_tone_offset_counter as u32))
            .unwrap();
        output_cursor
            .write_le(&u32::to_le_bytes(tone_size as u32))
            .unwrap();
        last_tone_offset_counter += tone_size as u32;
    }
    output_cursor
        .write(
            &n3b_data[(tone_offset + 12 + tone_count * 8) as usize
                ..(tone_offset + 8 + last_tone_offset + last_tone_size) as usize],
        )
        .unwrap();

    for x in new_entries.iter() {
        let name = x;
        output_cursor.write(source_pre_meta_data).unwrap();
        output_cursor
            .write(&u8::to_le_bytes(name.len() as u8 + 1))
            .unwrap();
        output_cursor.write(name.as_bytes()).unwrap();
        let mut counter = name.len() + 1;
        if counter % 4 == 0 {
            output_cursor.write_le(&u32::to_le_bytes(0)).unwrap();
        }
        while counter % 4 != 0 {
            output_cursor.write(&u8::to_le_bytes(0)).unwrap();
            counter += 1;
        }
        output_cursor.write_le(&u32::to_le_bytes(0)).unwrap();
        output_cursor.write_le(&u32::to_le_bytes(8)).unwrap();
        output_cursor.write_le(&u32::to_le_bytes(0)).unwrap();
        output_cursor.write_le(&u32::to_le_bytes(0x22E8)).unwrap();
        output_cursor
            .write(
                &n3b_data
                    [source_meta_offset as usize..(source_meta_offset + source_meta_size) as usize],
            )
            .unwrap();
    }

    output_cursor
        .write(&n3b_data[(tone_offset + 8 + last_tone_offset + last_tone_size) as usize..])
        .unwrap();

    Ok(output_cursor.into_inner())
}


pub fn read_u32(cursor: &mut std::io::Cursor<&mut [u8]>) -> u32 {
    let buf: &mut [u8; 4] = &mut [0; 4];
    cursor.read_exact(buf).unwrap();
    u32::from_le_bytes(*buf)
}

pub fn get_sub_meta_offset_and_size(cursor: &mut std::io::Cursor<&mut [u8]>) -> (u64, u64) {
    // Source Data offset, Source Data Size
    while cursor.position() % 4 != 0 {
        cursor.seek(std::io::SeekFrom::Current(1)).unwrap();
    }

    while read_u32(cursor) != 0x22E8 {}

    let start_pos = cursor.position();
    let mut break_counter = 0;
    loop {
        let val = read_u32(cursor);
        if break_counter % 2 == 0 {
            if val == 0 {
                break_counter += 1;
            } else {
                break_counter = 0;
            }
        } else {
            if val == 0xFFFFFFFF {
                break_counter += 1;
            } else {
                break_counter = 0;
            }
        }

        if break_counter == 8 {
            break;
        }
    }

    return (start_pos, cursor.position() - start_pos);
}