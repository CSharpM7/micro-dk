use crate::imports::imports_acmd::*;

#[acmd_script( agent = "donkey", script = "game_speciallwend", category = ACMD_GAME )]
unsafe fn game_speciallandinghi(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;

    frame(fighter.lua_state_agent, 62.0);
    if macros::is_excute(fighter) {
        /* 
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FREE,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);*/
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 60, 0, 70, 5.0, 0.0, 11.0, 20.0, Some(0.0), Some(11.0), Some(-4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "donkey", script = "effect_speciallwend", category = ACMD_EFFECT )]
unsafe fn effect_speciallwend(fighter: &mut L2CAgentBase) {

    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_XY);

        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 2, 13, 5, 19, 6, -33, 2, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    
    frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
    }
}
#[acmd_script( agent = "donkey", script = "sound_speciallwend", category = ACMD_SOUND )]
unsafe fn sound_speciallwend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_donkey_landing03"));
    }
    frame(fighter.lua_state_agent, 62.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
}
#[acmd_script( agent = "donkey", script = "expression_speciallwend", category = ACMD_EXPRESSION )]
unsafe fn expression_speciallwend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

//barrel_screw
#[acmd_script( agent = "donkey", script = "game_specialairlw", category = ACMD_GAME )]
unsafe fn game_specialairlw(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(boma, 0.0);
    }

}

#[acmd_script( agent = "donkey", script = "effect_specialairlw", category = ACMD_EFFECT )]
unsafe fn effect_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "donkey", script = "sound_specialairlw", category = ACMD_SOUND )]
unsafe fn sound_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_special_l01"));
    }
}
#[acmd_script( agent = "donkey", script = "expression_specialairlw", category = ACMD_EXPRESSION )]
unsafe fn expression_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_stg_power_up"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "donkey", script = "game_speciallwloop", category = ACMD_GAME )]
unsafe fn game_specialairlw_launch(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    let mut barrelBoma = fighter.module_accessor;

    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);

        //Attack can hit a full second later
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, 363, 60, 0, 70, 5.0, 0.0, 0.0, 0.0, None,None,None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 60, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}


#[acmd_script( agent = "donkey", script = "effect_speciallwloop", category = ACMD_EFFECT )]
unsafe fn effect_specialairlw_launch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "donkey", script = "sound_speciallwloop", category = ACMD_SOUND )]
unsafe fn sound_specialairlw_launch(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
    }
}
#[acmd_script( agent = "donkey", script = "expression_speciallwloop", category = ACMD_EXPRESSION )]
unsafe fn expression_specialairlw_launch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        effect_speciallwend,
        sound_speciallwend,
        expression_speciallwend,

        game_specialairlw,
        effect_specialairlw,
        sound_specialairlw,
        expression_specialairlw,

        game_specialairlw_launch,
        effect_specialairlw_launch,
        sound_specialairlw_launch,
        expression_specialairlw_launch,

        game_speciallandinghi,
    );
}