mod agent;
mod acmd;
mod status;
mod frame;

#[smashline::installer]
pub fn install() {
    acmd::install();
    status::install();
    agent::install();
    frame::install();
}