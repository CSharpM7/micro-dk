use crate::imports::imports_agent::*;

unsafe extern "C" fn special_hi_preU(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Check to make sure we dont have an item
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    if !StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        if ItemModule::is_have_item(fighter.module_accessor, 0)
        || crate::vars::BARREL_TIMER[entry]>0
        //|| is_HDR() 
        {
            println!("Nope");
            return false.into();
        }
    }
    return true.into();
}
unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_DONKEY {
        return;
    }
    let entry = get_entry_from_boma(fighter.module_accessor) as usize;
    crate::vars::BARREL_TIMER[entry] = 0;
    //fighter.global_table[USE_SPECIAL_HI_CALLBACK].assign(&L2CValue::Ptr(special_hi_preU as *const () as _));
}
#[smashline::fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    smashline::install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
}