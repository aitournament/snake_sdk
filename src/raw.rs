pub const DIRECTION_NORTH: u32 = 0;
pub const DIRECTION_EAST: u32 = 1;
pub const DIRECTION_SOUTH: u32 = 2;
pub const DIRECTION_WEST: u32 = 3;

pub const SPEED_NORMAL: u32 = 0;
pub const SPEED_FAST: u32 = 1;

pub const TYPE_EMPTY: u32 = 0;
pub const TYPE_FOOD: u32 = 1;
pub const TYPE_SNAKE_HEAD: u32 = 2;
pub const TYPE_SNAKE_BODY: u32 = 3;

pub const SPLIT_RESULT_FRONT: u32 = 0;
pub const SPLIT_RESULT_BACK: u32 = 1;

pub const ERR_OK:i32 = 0;
pub const ERR_COOL_DOWN:i32 = -1;

extern "C" {
    // constants
    pub fn get_arena_width() -> u32;
    pub fn get_arena_height() -> u32;
    pub fn get_cpu_cycles_per_tick() -> u32;
    // pub fn get_owner_id() -> u32;

    // actions
    pub fn set_direction(direction: u32);
    pub fn r#move();
    pub fn leap();
    pub fn sleep_remaining_tick();
    pub fn sleep(cycles: u64);
    pub fn split();
    pub fn suicide();

    // current position using "screen" coordinates (top-left is 0,0)
    pub fn get_current_pos(x_out: *mut u32, y_out: *mut u32);
    pub fn observe(x: u32, y: u32, out_type: *mut u32, out_0: *mut u32, out_1: *mut u32, out_2: *mut u32);

    pub fn get_length() -> u32;
    pub fn get_health() -> u32;
    pub fn get_current_tick() -> u64;
    pub fn get_current_cpu_cycle_in_tick() -> u64;
    pub fn rand(min: u32, max: u32) -> u32;
}