use crate::raw;
use crate::raw::{DIRECTION_EAST, DIRECTION_NORTH, DIRECTION_SOUTH, DIRECTION_WEST};
use core::hint::unreachable_unchecked;
use core::mem::MaybeUninit;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North = DIRECTION_NORTH as isize,
    East = DIRECTION_EAST as isize,
    South = DIRECTION_SOUTH as isize,
    West = DIRECTION_WEST as isize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Observation {
    Empty,
    Food(FoodInfo),
    SnakeHead(SnakeInfo),
    SnakeBody(SnakeInfo),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FoodInfo {
    pub health_value: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SnakeInfo {
    pub owner_id: u32,
    pub snake_id: u32,
    pub health: u32,
}

pub fn get_cpu_cycles_per_tick() -> u32 {
    unsafe { raw::get_cpu_cycles_per_tick() }
}

pub fn set_direction(direction: Direction) {
    unsafe {
        raw::set_direction(direction as u32);
    }
}

/// Move immediately (at the end of the current CPU cycle) in the current direction. A snake must move once per tick.
/// If the snake isn't manually moved, it will be automatically called at the end of the tick.
/// Note that this takes effect on the exact CPU cycle it is called. This means that all
/// snakes do not necessarily move at the same time (you can pick when during the tick the move occurs).
/// If this is incorrectly called, the snake will die from exhaustion.
pub fn move_snake() {
    unsafe { raw::r#move() };
}

/// A leap is an additional move that can occur at most once every 2 ticks.
/// It must only be called during a tick in which a move has already been performed, and
/// a leap was not performed during the previous tick.
/// Performing a leap will consume the last body part of this snake, which will drop on the floor as food.
/// If this is incorrectly called, the snake will die from exhaustion.
pub fn leap() {
    unsafe {
        raw::leap();
    }
}

pub fn sleep_remaining_tick() {
    unsafe {
        raw::sleep_remaining_tick();
    }
}

pub fn sleep(cycles: u64) {
    unsafe { raw::sleep(cycles) }
}

pub fn get_arena_width() -> u32 {
    unsafe { raw::get_arena_width() }
}

pub fn get_arena_height() -> u32 {
    unsafe { raw::get_arena_height() }
}

pub fn get_arena_size() -> (u32, u32) {
    (get_arena_width(), get_arena_height())
}

pub fn get_current_pos() -> (u32, u32) {
    let mut x = MaybeUninit::<u32>::uninit();
    let mut y = MaybeUninit::<u32>::uninit();
    unsafe {
        raw::get_current_pos(x.as_mut_ptr(), y.as_mut_ptr());
        (x.assume_init(), y.assume_init())
    }
}

pub fn get_current_tick() -> u64 {
    unsafe { raw::get_current_tick() }
}

pub fn get_current_cpu_cycle_in_tick() -> u64 {
    unsafe { raw::get_current_cpu_cycle_in_tick() }
}

pub fn suicide() {
    unsafe { raw::suicide() }
}

pub fn split() {
    unsafe { raw::split() }
}

pub fn observe(x: u32, y: u32) -> Observation {
    let mut type_out = MaybeUninit::<u32>::uninit();
    let mut out_0 = MaybeUninit::<u32>::uninit();
    let mut out_1 = MaybeUninit::<u32>::uninit();
    let mut out_2 = MaybeUninit::<u32>::uninit();
    unsafe {
        raw::observe(
            x,
            y,
            type_out.as_mut_ptr(),
            out_0.as_mut_ptr(),
            out_1.as_mut_ptr(),
            out_2.as_mut_ptr(),
        );
    }
    match unsafe { type_out.assume_init() } {
        raw::TYPE_EMPTY => Observation::Empty,
        raw::TYPE_FOOD => Observation::Food(FoodInfo {
            health_value: unsafe { out_0.assume_init() as i32 },
        }),
        raw::TYPE_SNAKE_HEAD => Observation::SnakeHead(SnakeInfo {
            owner_id: unsafe { out_0.assume_init() },
            snake_id: unsafe { out_1.assume_init() },
            health: unsafe { out_2.assume_init() },
        }),
        raw::TYPE_SNAKE_BODY => Observation::SnakeBody(SnakeInfo {
            owner_id: unsafe { out_0.assume_init() },
            snake_id: unsafe { out_1.assume_init() },
            health: unsafe { out_2.assume_init() },
        }),
        _ => unsafe { unreachable_unchecked() },
    }
}

pub fn get_length() -> u32 {
    unsafe { raw::get_length() }
}

pub fn get_health() -> u32 {
    unsafe { raw::get_health() }
}

pub fn rand(min: u32, max: u32) -> u32 {
    unsafe { raw::rand(min, max) }
}
