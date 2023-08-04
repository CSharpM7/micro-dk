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
    hash40,
    lua2cpp::*,
    phx::*
};
use smashline::*;

#[macro_use]
extern crate lazy_static;

mod dk;
mod imports;
pub mod data;
pub mod vars;
use data::gamemode::*;

//unsafe fn get_article_use_type_mask() -> u8 { 1 }
#[skyline::hook(offset = 0x3a6650)]
unsafe fn get_article_use_type_mask(weapon_kind: i32, entry_id: i32) -> u8 {
    let barrel_kind = *WEAPON_KIND_DONKEY_DKBARREL;
    if weapon_kind == barrel_kind{
        return 1;
    }
    call_original!(weapon_kind, entry_id)
}


#[skyline::main(name = "smashline_dk")]
pub fn main() {
    #[cfg(feature = "devhook")]{
        skyline::install_hooks!(
            get_article_use_type_mask
        ); 
        println!("[smashline_dk::main] Dev Hook Installed");
        return;
    }
    println!("[smashline_dk::main] Loading...");
    //data::install();
    data::gamemode::set_gamemode();
    dk::install();
    println!("[smashline_dk::main] Loaded!");
    
}