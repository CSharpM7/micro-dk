use crate::imports::imports_agent::*;

unsafe fn barrel_timer(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor,status: i32)
{
    let entry = get_entry_from_boma(boma) as usize;
    let mut current_time = crate::vars::BARREL_TIMER[entry];
    if (current_time>0)
    && status != *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY
    {
        current_time-=1;
        crate::vars::BARREL_TIMER[entry] = current_time;
        if (current_time==0)
        {
            let lr = PostureModule::lr(boma);
            let flash_y_offset = WorkModule::get_param_float(boma, hash40("height"), 0);
            if WorkModule::get_param_int(boma, hash40("param_motion"), hash40("flip")) != 0 {
                EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, flash_y_offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
            }
            else {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0 * lr, flash_y_offset, 2, 0, 0, 0, 1.0, true);
            }
            LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
        }
    }
}


unsafe fn barrel_air_despawn(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor,status: i32, motion: u64)
{
    if motion == Hash40::new("special_air_hi").hash {return;}

    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL)
    {
        let barrelBoma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
        let barrelFrame = MotionModule::frame(barrelBoma);
    
        if barrelFrame > 40.0
        {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        else if barrelFrame > 30.0{
            if (barrelFrame.floor() % 2.0 == 0.0)
            {
                ModelModule::set_alpha(barrelBoma, 0.5);
            }
            else
            {
                ModelModule::set_alpha(barrelBoma, 1.0);
            }
        }
    }
}


unsafe fn dk_update(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    barrel_timer(fighter,boma,status_kind);
    barrel_air_despawn(fighter,boma,status_kind,motion_kind);
}

#[fighter_frame( agent = FIGHTER_KIND_DONKEY )]
fn dk_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        dk_update(fighter);
    }
}
#[smashline::fighter_frame_callback]
fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let category = smash::app::utility::get_category(boma);
        let kind = smash::app::utility::get_kind(boma);
        if category == BATTLE_OBJECT_CATEGORY_FIGHTER && kind == FIGHTER_KIND_DONKEY {
            dk_update(fighter);
        }
    }
}


pub fn install() {
    //#[cfg((feature = "dev"))]
    smashline::install_agent_frame_callbacks!(
      global_fighter_frame
    );
    /* 
    #[cfg(not(feature = "dev"))]
    smashline::install_agent_frames!(
        samus_frame
    );*/
}