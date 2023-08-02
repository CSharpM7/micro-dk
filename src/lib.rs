#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    //lib::,
    app::{
        *,
        self,
        sv_animcmd::{
            frame,
            wait
        },
        lua_bind::*
    },
    hash40,
    lua2cpp::*,
    phx::*
};
use smash_script::{
    *,
    macros::*
};
use smashline::*;

#[macro_use]
extern crate lazy_static;

mod acmd;
mod frame;
mod status;
mod agent;
pub mod data;
pub mod util;
pub mod vars;
use util::*;
use data::gamemodes::*;

//unsafe fn get_article_use_type_mask() -> u8 { 1 }
#[skyline::hook(offset = 0x3a6650)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    let barrel_kind = *WEAPON_KIND_DONKEY_DKBARREL;
    if weapon_kind == barrel_kind{
        return 1;
    }
    println!("Weapon: {weapon_kind} Entry: {entry_id} Barrels: {barrel_kind}");
    call_original!(weapon_kind, entry_id)
}

#[skyline::main(name = "smashline_dk")]
pub fn main() {
    println!("[smashline_dk::main] Loading...");
    data::install();
    data::gamemodes::set_gamemode();
    acmd::install();
    frame::install();
    status::install();
    agent::install();
    println!("[smashline_dk::main] Loaded!");
    
    skyline::install_hooks!(
        get_article_use_type_mask
    );
    
}