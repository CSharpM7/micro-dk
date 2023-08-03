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
    if (StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
    {
        if (MotionModule::frame(fighter.module_accessor) > 1.0
        && MotionModule::frame(fighter.module_accessor) < 61.0)
        {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_landing_hi"), 61.0, 1.0, false, 0.0, false, false);
        }
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

            let speed_max = 0.5;
            let accel = 0.125;
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_max,
                0.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_max,
                0.0
            );
            sv_kinetic_energy!(
                controller_set_accel_x_add,
                fighter,
                accel
            );

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
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_landing_hi"), 61.0, 1.0, false, 0.0, false, false);
    }

}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dk_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_model_visible(fighter.module_accessor,  true);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        macros::EFFECT(fighter, Hash40::new("donkey_entry"), Hash40::new("top"), 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    return original!(fighter);
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dk_speciallw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
            *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    return false.into();
}
#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn dk_speciallw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_RESET);

    if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        let speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("x_spd_max_air"));
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        let accel_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("x_acl_air"));
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            accel_max
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );

        let speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("y_spd_air"));
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );

        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y
        );
        let mut accel = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"),0);
        let mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("y_acl_mul"));
        accel*=mul;
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel
        );

    }
    
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    return false.into();
}


#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dk_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
    speciallw_main_helper(fighter);
    if StopModule::is_stop(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_CLIFF_CHECK) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_CLIFF_CHECK);
            fighter.sub_fighter_cliff_check(0xa0.into());
        }
    }
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {Hash40::new("special_lw")} else {Hash40::new("special_air_lw")};
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn speciallw_main_helper(fighter: &mut L2CFighterCommon) {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE){
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
            let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("landing_frame"));
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE);
        }
        else{
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
            EFFECT_OFF_KIND(fighter,Hash40::new_raw(0xd6dd4defe),false,false);
        }
    }
    else{
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_MOT_FRAME){
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
        }
        else{
            AttackModule::clear_all(fighter.module_accessor);
            let ground_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("ground_mot_frame"));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), ground_frame-1.0, 1.0, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND);
        }
    }
}
unsafe extern "C" fn speciallw_main_helper2(fighter: &mut L2CFighterCommon) {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let mut accel = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"),0); //112
    let mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("y_acl_mul")); //128

    let param_max = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {hash40("x_spd_max_ground")} else {hash40("x_spd_max_air")};
    let speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),param_max);
    let param_acl = if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {hash40("x_acl_ground")} else {hash40("x_acl_air")};
    let accel_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),param_acl);

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT){
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel*mul
        );
    }
    else{
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel
        );
    }
    if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_mul,
            fighter,
            accel_max
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );

        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    }
    else{
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_DASH,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_GROUND_SPINEND){
            sv_kinetic_energy!(
                controller_set_accel_x_mul,
                fighter,
                accel_max
            );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_max,
                0.0
            );
        }
        else{
            sv_kinetic_energy!(
                controller_set_accel_x_mul,
                fighter,
                0.0
            );
            sv_kinetic_energy!(
                set_stable_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                0.0,
                0.0
            );
            let brake = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("x_dcl_ground_spinend")); //128
            sv_kinetic_energy!(
                set_brake, 
                fighter, 
                FIGHTER_KINETIC_ENERGY_ID_CONTROL, 
                brake, 
                0.0
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                speed_max,
                0.0
            );

            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        speed_x,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.0
    );
}
unsafe extern "C" fn speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    
    if StatusModule::is_changing(fighter.module_accessor) {
        speciallw_main_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        return 0.into();
    }
    else{
        speciallw_main_helper2(fighter);
    }
    return 0.into();
}

pub fn install() {
    install_status_scripts!(
        dk_specialhi_pre,
        dk_specialhi_exec,
        dk_specialhi_end,

        dk_speciallw_init,
        dk_speciallw_pre,
        dk_speciallw_main,
    );
}