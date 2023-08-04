use crate::imports::imports_acmd::*;

#[acmd_script( agent = "donkey", script = "game_attacklw4", category = ACMD_GAME)]
unsafe fn game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 9.0+5.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 78, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 84, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 84, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 78, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.58);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 78, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 84, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new("top"), 14.0, 84, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new("top"), 14.0, 78, 30, 0, 85, 10.3, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.58);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        //WorkModule::on_flag(agent.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }

}
#[acmd_script( agent = "donkey", script = "effect_attacklw4", category = ACMD_EFFECT)]
unsafe fn effect_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0+5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0+16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("donkey_handslap"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 6, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}
#[acmd_script( agent = "donkey", script = "sound_attacklw4", category = ACMD_SOUND)]
unsafe fn sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_common_smash_start"));
    }
    frame(agent.lua_state_agent, 9.0+5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_donkey_smash_l01"));
    }
    frame(agent.lua_state_agent, 9.0+10.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_donkey_smash_l02"));
    }
}
#[acmd_script( agent = "donkey", script = "expression_attacklw4", category = ACMD_EXPRESSION)]
unsafe fn expression_attacklw4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        //slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0+2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 9.0+5.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 9.0+16.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        //slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}
#[acmd_script( agent = "donkey", script = "expression_attacklw4charge", category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_attacklw4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        physics!(agent,*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.8, -1, 0.8, 0.8, -1, Hash40::new("toer2"));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attacklw4,
        effect_attacklw4,
        sound_attacklw4,
        expression_attacklw4,

        expression_attacklw4charge,
    );
}