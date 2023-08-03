mod agent;
mod acmd;
mod status;
mod frame;

#[smashline::installer]
pub fn install_dev() {
    acmd::install();
    status::install();
    agent::install();
}

pub fn install() {
    frame::install();
    
    #[cfg(not(feature = "dev"))]{
        acmd::install();
        agent::install();
        status::install();
    }
    
    #[cfg(feature = "dev")]{
        install_dev();
    }
}