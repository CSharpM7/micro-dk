use crate::imports::imports_agent::*;

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_ATTACK_LW4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dk_attacklw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.attack_lw4_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(dk_attacklw4_main_loop as *const () as _))
    //fighter.main_shift(dk_attacklw4_main_loop)
}

unsafe fn dk_attacklw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let offs = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"),hash40("offs")); //152
    let offs_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"),hash40("offs_y")); //fVar10
    let length = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"),hash40("length")); //184
    let scale = PostureModule::scale(fighter.module_accessor); //328
    let mut pos_x =  PostureModule::pos_x(fighter.module_accessor); //232
    let mut pos_y =  PostureModule::pos_y(fighter.module_accessor); //408
    let mut pos_z =  PostureModule::pos_z(fighter.module_accessor); //248
    let lr =  PostureModule::lr(fighter.module_accessor); //456
    //true 376
    let rot_z = PostureModule::rot_z(fighter.module_accessor,0).to_radians(); //248
    //0.0 290
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK){
    //if AttackModule::is_attack(fighter.module_accessor, 0, false){
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32){
            println!("RotZ: {rot_z}");
            for i in 0..4 {
                if AttackModule::is_attack(fighter.module_accessor, i, false){
                    let shift = if {lr>0.0} {2.0} else {-2.0};
                    let mut attack_x = ((length*1.5) + (offs*(i as f32)) - (length*1.25*i as f32)) + (offs/2.0);
                    let mut attack_y = offs_y;//+(offs_y*rot_z.sin());
                    let attack_pos_x = attack_x* rot_z.cos();
                    let attack_pos_y = attack_y + (attack_y*rot_z.sin());
                    let mut ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
                    
                    if (GroundModule::ray_check_hit_pos(fighter.module_accessor, &smash::phx::Vector2f{ x: pos_x+attack_pos_x*lr, y: pos_y+attack_pos_y}, &Vector2f{ x: rot_z.sin()*offs_y, y: -(rot_z.cos()*offs_y*2.0)}, ground_hit_pos, true) != 1)
                    && [0,3].contains(&i) {
                        let lastpos = GroundModule::get_latest_down_correct_pos(fighter.module_accessor);
                        attack_x = lastpos.x;
                        attack_y = lastpos.y+offs_y;
                    }

                    /* 
                    if (GroundModule::check_down_correct_pos(fighter.module_accessor, Vector2f{x: attack_x,y:attack_y}) & 1 )==0 {
                        let ground_hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
                        //let maxheight = (i as f32-2.0).abs();
                        let maxheight = 2.0;//attack_y*rot_z.cos();

                        EFFECT(fighter,Hash40::new("sys_vector"),Hash40::new("top"),attack_x,attack_y+maxheight,0,0,0,rot_z.to_degrees()+90.0,2.0,0,0,0,0,0,0,true);

                        if GroundModule::ray_check_hit_pos(fighter.module_accessor, &smash::phx::Vector2f{ x: pos_x+attack_x, y: pos_y+attack_y+maxheight*1.0}, &Vector2f{ x: rot_z.sin()*maxheight, y: -(offs_y+maxheight*1.5)}, ground_hit_pos, true) == 1
                        {
                            //attack_x = ground_hit_pos.x-pos_x;
                            attack_y = ground_hit_pos.y+offs_y-pos_y;
                            println!("Diff ground");
                        }
                        else{
                            let lastpos = GroundModule::get_latest_down_correct_pos(fighter.module_accessor); //x:504, y:296,z:248
                            attack_x = lastpos.x-pos_x;
                            attack_y = lastpos.y-pos_y+offs_y;
                            println!("No ground");
                        }
                    }*/
                    println!("Hitbox {i}: Attack X: {attack_x} Attack Y: {attack_y}");
                    AttackModule::set_offset(fighter.module_accessor, i, &Vector3f{x: pos_z,y:attack_y,z:attack_x});
                }
            }
        }
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK);
    }
    if AttackModule::is_attack(fighter.module_accessor, 0, false){
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_vector"),false,false);
    }
    
    
    fighter.status_AttackLw4_Main()
}
unsafe fn dk_attacklw4_exec_original(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    lib::L2CValue::L2CValue(aLStack152,0);
    lib::L2CValue::L2CValue(aLStack168,0);
    lib::L2CValue::L2CValue(aLStack184,0);
    lib::L2CValue::L2CValue(aLStack200,0);
    lib::L2CValue::L2CValue(aLStack216,0);
    lib::L2CValue::L2CValue(aLStack232,0);
    lib::L2CValue::L2CValue(aLStack248,0);
    lib::L2CValue::L2CValue(aLStack264,0);
    lib::L2CValue::L2CValue(aLStack280,0);
    lib::L2CValue::L2CValue(aLStack296,0);
    lib::L2CValue::L2CValue(aLStack312,0);
    lib::L2CValue::L2CValue(aLStack328,0);
    lib::L2CValue::L2CValue(aLStack344,0);
    lib::L2CValue::L2CValue(aLStack360,0);
    lib::L2CValue::L2CValue(aLStack376,false);
    lib::L2CValue::L2CValue(aLStack392,0);
    lib::L2CValue::L2CValue(aLStack408,0);
    lib::L2CValue::L2CValue(aLStack424,0);
    lib::L2CValue::L2CValue(aLStack440,0);
    lib::L2CValue::L2CValue(aLStack456,0);
    lib::L2CValue::L2CValue(aLStack472,0);
    lib::L2CValue::L2CValue(aLStack488,0);
    lib::L2CValue::L2CValue(aLStack504,0);
    */
    let offs = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"),hash40("offs")); //152
    let offs_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"),hash40("offs_y")); //fVar10
    let length = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"),hash40("length")); //184
    let scale = PostureModule::scale(fighter.module_accessor); //328
    let mut pos_x =  PostureModule::pos_x(fighter.module_accessor); //232
    let mut pos_y =  PostureModule::pos_y(fighter.module_accessor); //408
    let mut pos_z =  PostureModule::pos_z(fighter.module_accessor); //248
    let lr =  PostureModule::lr(fighter.module_accessor); //456
    //true 376
    let rot_z = PostureModule::rot_z(fighter.module_accessor,0).to_radians(); //248
    //0.0 290
    if true {

    }
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_DONKEY_STATUS_SPECIAL_LW_FLAG_ATTACK){
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32){
            for i in 0..4 {
                if AttackModule::is_attack(fighter.module_accessor, i, true){
                    let mut alstack440 = offs*lr;//152? 290?
                    let l536 = length*i as f32; //184? 536?
                    let l552 = l536*1.5; //552
                    let mut alstack200 = l536-l552; //200
                    alstack200 += alstack440;
                    alstack200*=scale;

                    if false {

                    }
                    else{
                        let al504=pos_x+alstack200;
                        let al296 = al504;
                    }
                    if (GroundModule::check_down_correct_pos(fighter.module_accessor, Vector2f{x: 0.0,y:rot_z}) & 1 ) !=0 {
                        let latestpos = GroundModule::get_latest_down_correct_pos(fighter.module_accessor); //x:504, y:296,z:248
                        pos_x = latestpos.x;
                        pos_y = latestpos.y;
                    }

                }
            }
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dk_attacklw4_main,
    );
}