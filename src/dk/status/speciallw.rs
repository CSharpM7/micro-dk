use crate::imports::imports_agent::*;

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
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("landing_frame"));
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_MOT_CHANGE){
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
            //let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"),hash40("landing_frame"));
            //WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
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
    
    if MotionModule::is_end(fighter.module_accessor) {
        let status_g = if MotionModule::motion_kind(fighter.module_accessor) == Hash40::new("special_lw_air").hash {FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into()} else {FIGHTER_STATUS_KIND_WAIT.into()};
        fighter.change_status_by_situation(status_g, FIGHTER_STATUS_KIND_FALL_SPECIAL.into(),false.into());
        return 0.into();
    }
    if StatusModule::is_changing(fighter.module_accessor) {
        speciallw_main_helper(fighter);
    }
    speciallw_main_helper2(fighter);
    return 0.into();
}

pub fn install() {
    install_status_scripts!(
        dk_speciallw_init,
        dk_speciallw_pre,
        dk_speciallw_main,
    );
}