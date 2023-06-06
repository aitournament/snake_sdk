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
pub enum ObservationItem {
    Food(FoodInfo),
    SnakeHead(SnakeInfo),
    SnakeBody(SnakeInfo),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Observation {
    /// The amount of damage to a snakes health if the head of a snake
    /// is on top of poison at the end of a tick
    pub poison: u32,

    /// The item at this location
    pub item: Option<ObservationItem>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FoodInfo {
    /// The amount of health that a snake will gain by eating this food.
    pub health_value: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SnakeInfo {
    pub team_id: u32,
    pub snake_id: u32,
    pub health: u32,
}

/// Get the number of CPU cycles executed per tick. This is constant.
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
pub fn move_snake() {
    unsafe { raw::r#move() };
}

/// A leap is an additional move that can occur at most once every 2 ticks.
/// It must only be called during a tick in which a move has already been performed, and
/// a leap was not performed during the current or previous tick.
/// Performing a leap will consume the last body part of this snake, which will drop on the floor as poison.
pub fn leap() {
    unsafe {
        raw::leap();
    }
}

/// Sleep (do nothing) for all remaining CPU cycles in the current tick.
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

/// Get the number of CPU cycles that have already executed in the current tick.
pub fn get_current_cpu_cycle_in_tick() -> u64 {
    unsafe { raw::get_current_cpu_cycle_in_tick() }
}

/// Kill the snake immediately. All parts of the snake will turn into poison.
pub fn suicide() {
    unsafe { raw::suicide() }
}

/// Logs a message. The maximum length is 50. Any message longer than this will be truncated.
/// A snake can send at most 10 logs per tick. Any additional logs will be ignored.
pub fn speak(msg: &str) {
    unsafe { raw::speak(msg.as_ptr(), msg.len() as u32) }
}

/// If the snake has a length of at least 9, it is eligible to split into two.
/// The snake is split into three parts (the middle is rounded up, the rest down). The first part will remain as
/// the original snake. The middle part will be lost and turn into poison. The end will become a new
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
    const OUT_TYPE: usize = 0;
    const OUT_TEAM_ID: usize = 1;
    const OUT_SNAKE_ID: usize = 2;
    const OUT_HEALTH: usize = 3;
    const OUT_POISON: usize = 4;

    let mut out = MaybeUninit::<[u32; 5]>::uninit();
    let out = unsafe {
        raw::observe(x, y, out.as_mut_ptr());
        out.assume_init()
    };

    let poison = out[OUT_POISON];
    let item = match out[OUT_TYPE] {
        raw::TYPE_EMPTY => None,
        raw::TYPE_FOOD => Some(ObservationItem::Food(FoodInfo {
            health_value: out[OUT_HEALTH],
        })),
        raw::TYPE_SNAKE_HEAD => Some(ObservationItem::SnakeHead(SnakeInfo {
            team_id: out[OUT_TEAM_ID],
            snake_id: out[OUT_SNAKE_ID],
            health: out[OUT_HEALTH],
        })),
        raw::TYPE_SNAKE_BODY => Some(ObservationItem::SnakeBody(SnakeInfo {
            team_id: out[OUT_TEAM_ID],
            snake_id: out[OUT_SNAKE_ID],
            health: out[OUT_HEALTH],
        })),
        _ => unsafe { unreachable_unchecked() },
    };

    Observation { poison, item }
}

/// Get the length of the snake.
pub fn get_length() -> u32 {
    unsafe { raw::get_length() }
}

/// Get the current health of the snake. The maximum health is 100. The health decreases by 1 each tick.
/// If the health of a snake ever reaches zero, it will die. A snake can gain health by eating food.
/// Food increases health. Poison damages health.
pub fn get_health() -> u32 {
    unsafe { raw::get_health() }
}

/// Get a random value between the min (inclusive) and max (exclusive).
/// The returned value is deterministic based on the current state of the game and the initial seed.
pub fn rand(min: u32, max: u32) -> u32 {
    unsafe { raw::rand(min, max) }
}
