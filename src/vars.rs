pub const BARREL_SPEED: f32 = 3.0;
pub const BARREL_SPEED_Y_MUL: f32 = 0.9;
pub const BARREL_CONTROL_X_MUL: f32 = 1.25;
pub const BARREL_TIMER_MAX: i32 = 180;
pub static mut BARREL_TIMER : [i32; 8] = [0; 8];
pub static mut BARREL_ANGLE : [f32; 8] = [0.0; 8];