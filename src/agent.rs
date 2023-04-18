use super::*;

pub const CHECK_SPECIAL_HI_UNIQ:            i32 = 0x3A;


unsafe extern "C" fn special_hi_preU(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry = get_entry_from_boma(fighter.module_accessor);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
    {
        return true.into();
    }
    else
    {
        if ItemModule::is_have_item(fighter.module_accessor, 0)
        || vars::BARREL_TIMER[entry]>0{
            return false.into();
        }
        else
        {
            return true.into();
        }
    }
}
unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_DONKEY {
        return;
    }
    let entry = get_entry_from_boma(fighter.module_accessor);
    vars::BARREL_TIMER[entry] = 0;
    fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(special_hi_preU as *const () as _));

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