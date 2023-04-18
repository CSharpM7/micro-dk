use super::super::*;

#[acmd_script( agent = "donkey", script = "game_specialhi", category = ACMD_GAME )]
unsafe fn game_specialhi(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;

    if macros::is_excute(fighter) {
        let entry = get_entry_from_boma(boma);
        vars::BARREL_TIMER[entry] = vars::BARREL_TIMER_MAX;
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL),0,0,false,false);
        StatusModule::change_status_force(fighter.module_accessor,*FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP,false);
    }
}

#[acmd_script( agent = "donkey", script = "game_speciallandinghi", category = ACMD_GAME )]
unsafe fn game_speciallandinghi(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;

    frame(fighter.lua_state_agent, 62.0);
    if macros::is_excute(fighter) {
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
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "donkey", script = "effect_specialhi", category = ACMD_EFFECT )]
unsafe fn effect_specialhi(fighter: &mut L2CAgentBase) {

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
#[acmd_script( agent = "donkey", script = "sound_specialhi", category = ACMD_SOUND )]
unsafe fn sound_specialhi(fighter: &mut L2CAgentBase) {
    
    frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_donkey_landing01"));
    }
}

//barrel_screw
#[acmd_script( agent = "donkey", script = "game_specialairhi", category = ACMD_GAME )]
unsafe fn game_specialairhi(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    let mut barrelBoma = fighter.module_accessor;

    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_model_visible(boma, false);

        ArticleModule::generate_article(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL,true,0);

        barrelBoma = get_article_boma(boma, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);

        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 
            Hash40::new("special_air_hi"),
        true, 0.0);
        PostureModule::set_lr(barrelBoma, 1.0);
        PostureModule::update_rot_y_lr(barrelBoma);
        PostureModule::set_scale(barrelBoma,PostureModule::scale(fighter.module_accessor)*1.2,false);
        PostureModule::add_pos(barrelBoma, &Vector3f{x: 0.0, y:8.6*PostureModule::scale(boma), z:0.0});
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(boma, 0.0);
    }

}

#[acmd_script( agent = "donkey", script = "effect_specialairhi", category = ACMD_EFFECT )]
unsafe fn effect_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_item_arrival"), Hash40::new("top"), 0, 8, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}
#[acmd_script( agent = "donkey", script = "sound_specialairhi", category = ACMD_SOUND )]
unsafe fn sound_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_special_h02"));
    }

}

#[acmd_script( agent = "donkey", script = "game_specialairhilaunch", category = ACMD_GAME )]
unsafe fn game_specialairhi_launch(fighter: &mut L2CAgentBase) {
    let boma = fighter.module_accessor;
    let mut barrelBoma = fighter.module_accessor;

    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_specialhi,
        effect_specialhi,
        sound_specialhi,

        game_specialairhi,
        effect_specialairhi,
        sound_specialairhi,

        game_specialairhi_launch,
        game_speciallandinghi,
    );
}