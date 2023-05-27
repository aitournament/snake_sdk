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
    /// The amount of health that a snake will gain (or lose) by eating this food.
    pub health_value: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SnakeInfo {
    pub team_id: u32,
    pub snake_id: u32,
    pub health: u32,
}

/// Gets the number of CPU cycles executes per tick. This is constant.
pub fn get_cpu_cycles_per_tick() -> u32 {
    unsafe { raw::get_cpu_cycles_per_tick() }
}

/// Sets the direction of the snake. This is the direction the snake will move next.
pub fn set_direction(direction: Direction) {
    unsafe {
        raw::set_direction(direction as u32);
    }
}

/// Move immediately in the current direction. A snake must move once per tick.
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

/// Sleep (do nothing) for all remaining CPU cycles in the current tick. There is no difference in sleeping
/// vs "busy waiting" manually, this is just for convenience.
pub fn sleep_remaining_tick() {
    unsafe {
        raw::sleep_remaining_tick();
    }
}

/// Sleep (do nothing) for a specific number of cycles.
pub fn sleep(cycles: u64) {
    unsafe { raw::sleep(cycles) }
}

/// Get the width of the arena. This is constant and does not change during a game.
pub fn get_arena_width() -> u32 {
    unsafe { raw::get_arena_width() }
}

/// Get the height of the arena. This is constant and does not change during a game.
pub fn get_arena_height() -> u32 {
    unsafe { raw::get_arena_height() }
}

/// Get the (width, height) of the arena. This is constant and does not change during a game.
pub fn get_arena_size() -> (u32, u32) {
    (get_arena_width(), get_arena_height())
}

/// Get the current position of the head of the snake.
pub fn get_current_pos() -> (u32, u32) {
    let mut x = MaybeUninit::<u32>::uninit();
    let mut y = MaybeUninit::<u32>::uninit();
    unsafe {
        raw::get_current_pos(x.as_mut_ptr(), y.as_mut_ptr());
        (x.assume_init(), y.assume_init())
    }
}

/// Get the current tick. Starts at 0.
pub fn get_current_tick() -> u64 {
    unsafe { raw::get_current_tick() }
}

/// Gets the number of CPU cycles that have already executed in the current tick.
pub fn get_current_cpu_cycle_in_tick() -> u64 {
    unsafe { raw::get_current_cpu_cycle_in_tick() }
}

/// Kill the snake immediately. All parts of the snake will turn into poisonous food.
pub fn suicide() {
    unsafe { raw::suicide() }
}

/// If the snake has a length of at least 9, it is eligible to split into two.
/// The snake is split into three parts (the middle is rounded up, the rest down). The first part will remain as
/// the original snake. The middle part will be lost and turn into (poisonous) food. The end will become a new
/// snake. The snake runtime will be forked, and both new snakes will continue to run independently.
/// You can use `get_id` to determine which snake is now running.
pub fn split() {
    unsafe { raw::split() }
}

/// Get the id of the snake. This is constant, except after a `split`, where the new snake will have a new id.
pub fn get_id() -> u32 {
    unsafe { raw::get_id() }
}

/// Get the id of your team. This is constant.
pub fn get_team_id() -> u32 {
    unsafe { raw::get_team_id() }
}

/// View a certain position in the arena.
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
            team_id: unsafe { out_0.assume_init() },
            snake_id: unsafe { out_1.assume_init() },
            health: unsafe { out_2.assume_init() },
        }),
        raw::TYPE_SNAKE_BODY => Observation::SnakeBody(SnakeInfo {
            team_id: unsafe { out_0.assume_init() },
            snake_id: unsafe { out_1.assume_init() },
            health: unsafe { out_2.assume_init() },
        }),
        _ => unsafe { unreachable_unchecked() },
    }
}

/// Get the length of the snake.
pub fn get_length() -> u32 {
    unsafe { raw::get_length() }
}

/// Get the current health of the snake. The maximum health is 100. The health decreases by 1 each tick.
/// If the health of a snake ever reaches zero, it will die. A snake can gain health by eating food.
/// Food has a health value, which is the amount health will increase. Food can have a negative health
/// value (if it's poisonous).
pub fn get_health() -> u32 {
    unsafe { raw::get_health() }
}

/// Get a random value between the min (inclusive) and max (exclusive).
/// The returned value is deterministic based on the current state of the game and the initial seed.
pub fn rand(min: u32, max: u32) -> u32 {
    unsafe { raw::rand(min, max) }
}
