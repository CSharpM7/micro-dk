mod specialbarrel;
mod specialspin;
mod attacklw4;

pub fn install() {
    specialbarrel::install();
    specialspin::install();
    attacklw4::install();
}