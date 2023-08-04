use crate::imports::imports_agent::*;

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dk_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
    {
        return original!(fighter);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_AIR_STOP,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return false.into();
}


#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn dk_specialhi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    ModelModule::set_visibility(fighter.module_accessor, false);
    
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND){
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_landing_hi"), 61.0, 1.0, false, 0.0, false, false);
            //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
            
            let entry = get_entry_from_boma(fighter.module_accessor) as usize;
            let angle = crate::vars::BARREL_ANGLE[entry];
            let speed= 3.0;
            let speed_x= (angle.to_radians()).sin()*speed;
            let lr = PostureModule::lr(fighter.module_accessor);
            SET_SPEED_EX(fighter,speed_x*lr,0.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
    {
        let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("x_dcl_ground_spinend")); //128
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_brake, 
            fighter, 
            FIGHTER_KINETIC_ENERGY_ID_CONTROL, 
            brake, 
            0.0
        );
    }
    else{
        let start = Hash40::new("special_air_hi").hash;
        let motion = MotionModule::motion_kind(fighter.module_accessor);
        if (motion != start){
            dk_specialhi_launch_exec(fighter);
        }
        else
        {
            dk_specialhi_start_exec(fighter);
        }
    }
    return false.into();
}

unsafe fn dk_specialhi_start_exec(fighter: &mut L2CFighterCommon) {

    //Freeze in place//
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP
    );
    sv_kinetic_energy!(
        clear_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    let frame = MotionModule::frame(fighter.module_accessor);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL){
        if frame > 2.0 {
            StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        }
        return;
    }
    let barrelBoma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
    let barrelMotion = MotionModule::motion_kind(barrelBoma);
    let barrelFrame = MotionModule::frame(barrelBoma);
    let canLaunch = barrelMotion == Hash40::new("special_air_hi_aim").hash;
    if (frame <10.0)
    {
        //Either being launch, or rotate barrel
        if canLaunch
        {
            if (ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) != 0
            {
                MotionModule::set_frame(fighter.module_accessor, 10.0, false);
                MotionModule::set_rate(fighter.module_accessor, 1.0);

                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 
                    Hash40::new("special_air_hi_launch"),
                true, 0.0);
                
                let angle = barrelFrame-45.0;
                println!("Request launch at angle {}",angle);

                let entry = get_entry_from_boma(fighter.module_accessor) as usize;
                crate::vars::BARREL_ANGLE[entry] = angle;
                PostureModule::set_rot(barrelBoma, &Vector3f{x: angle, y:0.0, z:0.0}, 0);
            }

            if MotionModule::is_end(barrelBoma)
            || barrelFrame <= 0.0
            || barrelFrame >= 90.0
            {
                let rate = MotionModule::rate(barrelBoma);
                ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, -rate);
            }
        }
        //Begin rotation
        else if MotionModule::is_end(barrelBoma)
        || barrelFrame >= 12.0
        {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 
                Hash40::new("special_air_hi_aim"),
            true, 45.0);
            let lr = if (*PostureModule::pos(fighter.module_accessor)).x >= 0.0 {-1.0} else {1.0};
            ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, 3.0*lr);
        }

    }
    else {
        let entry = get_entry_from_boma(fighter.module_accessor) as usize;
        let angle = crate::vars::BARREL_ANGLE[entry];

        if frame >= 28.0{
            macros::EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            macros::PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));

            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_launch"), 0.0, 1.0, false, 0.0, false, false);
            VisibilityModule::set_model_visible(fighter.module_accessor, true);
            
            if (angle.abs()>1.0) {
                PostureModule::set_lr(fighter.module_accessor, angle.signum());
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }

            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_SUPER_JUMP_PUNCH_AIR);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

            let speed= 3.0;
            let speed_x= (angle.to_radians()).sin()*speed;
            let speed_y= (angle.to_radians()).cos()*(speed+0.5);
            let lr = PostureModule::lr(fighter.module_accessor);
            println!("Launch! SpeedX: {} SpeedY: {}",speed_x,speed_y);
            SET_SPEED_EX(fighter,speed_x*lr,speed_y,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        }

    }
}

unsafe fn dk_specialhi_launch_exec(fighter: &mut L2CFighterCommon){

    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    let angle = crate::vars::BARREL_ANGLE[entry];
    let speed_x= (angle.to_radians()).sin()*3.0;
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let min_y = -10.0;
    let lr = PostureModule::lr(fighter.module_accessor);
    SET_SPEED_EX(fighter,speed_x*lr,(speed_y-0.07).max(min_y),*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32){
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND),false);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_landing_hi"), 61.0, 1.0, false, 0.0, false, false);
    }

}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn dk_specialhi_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_model_visible(fighter.module_accessor,  true);

    let start = Hash40::new("special_air_hi").hash;
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion == start {
        macros::EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        };
    }
    return original!(fighter);
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_CHECK_DAMAGE)]
unsafe fn dk_specialhi_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_model_visible(fighter.module_accessor,  true);

    let start = Hash40::new("special_air_hi").hash;
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion == start {
        macros::EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        };
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dk_specialhi_pre,
        dk_specialhi_exec,
        dk_specialhi_exit,
        //dk_specialhi_damage,
    );
}