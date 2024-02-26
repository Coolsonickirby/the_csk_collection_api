#![allow(arithmetic_overflow)]

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
        pub fn csk_collection_version() -> u16;
        pub fn csk_collection_installed() -> bool;
    }
}

pub fn play_bgm(ui_bgm_hash: u64){
    unsafe {
        externed::play_bgm(ui_bgm_hash);
    }
}

pub fn get_color_from_entry_id(entry_id: u32){
    unsafe {
        externed::get_color_from_entry_id(entry_id);
    }
}

pub fn change_entry_chara_ui(entry_id: u32, ui_chara_hash: u64, color_slot: u8){
    unsafe {
        externed::change_entry_chara_ui(entry_id, ui_chara_hash, color_slot);
    }
}

pub fn get_ui_chara_from_entry_id(entry_id: u32) -> u64{
    unsafe {
        externed::get_ui_chara_from_entry_id(entry_id)
    }
}

pub fn get_victor_color(){
    unsafe {
        externed::get_victor_color();
    }
}

pub fn allow_ui_chara_hash_online(ui_chara_hash: u64){
    unsafe {
        externed::allow_ui_chara_hash_online(ui_chara_hash);
    }
}

pub fn disable_ui_chara_hash_online(ui_chara_hash: u64){
    unsafe {
        externed::disable_ui_chara_hash_online(ui_chara_hash);
    }
}

pub fn is_online() -> bool{
    unsafe {
        externed::is_online()
    }
}

pub fn get_plugin_version() -> Version {
    unsafe {
        let res = externed::csk_collection_version();
        Version {
            major: (res >> 16) as _,
            minor: (res >> 8 & 0xFF) as _,
            patch: (res & 0xFF) as _,
        }
    }
}

pub fn csk_collection_installed() -> bool {
    unsafe {
        if (externed::csk_collection_installed as *const ()).is_null() {
            false
        } else {
            externed::csk_collection_installed()
        }
    }
}

pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8
}