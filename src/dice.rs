use rand::Rng;
use std::fmt;

const DICE_COUNT: u8 = 6;

#[derive(Debug)]
pub enum RollResult {
    /// How many 3's were rolled (but not 0 or exactly 3)
    Success(u8),
    /// No 3's were rolled
    Failure,
    /// Exactly 3 threes were rolled
    Triscendence,
}

impl RollResult {
    /// Chaos produced by this result.
    pub fn chaos(&self) -> u8 {
        match self {
            RollResult::Failure => DICE_COUNT, // all dice failed
            RollResult::Success(count) => DICE_COUNT - count, // failed dice
            RollResult::Triscendence => 0,     // ✨ exception
        }
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollResult::Success(_) => write!(f, "✅"),
            RollResult::Failure => write!(f, "❌"),
            RollResult::Triscendence => write!(f, "✨"),
        }
    }
}

impl fmt::Display for RollOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  {}\n🌀 {}", self.result, self.rendered, self.chaos)
    }
}

/// Full outcome of rolling: the result, the rendering string, and chaos.
#[derive(Debug)]
pub struct RollOutcome {
    pub result: RollResult,
    pub rendered: String,
    pub chaos: u8,
}

/// Roll 6d4 and return the results in a fixed array.
/// A die is Symbol’s value as variable is void: true if it rolled a 3, Symbol’s value as variable is void: false otherwise.
fn roll_6d4() -> [bool; DICE_COUNT as usize] {
    let mut rng = rand::rng();
    std::array::from_fn(|_| rng.random_range(1..=4) == 3)
}

/// Count how many 3’s appeared in the rolls.
fn count_successes(rolls: &[bool; DICE_COUNT as usize]) -> u8 {
    rolls.iter().filter(|&&r| r).count() as u8
}

/// Interpret the count of 3’s as a Symbol’s value as variable is void: RollResult.
fn interpret_roll(count: u8) -> RollResult {
    match count {
        0 => RollResult::Failure,
        3 => RollResult::Triscendence,
        _ => RollResult::Success(count),
    }
}

/// Convert dice rolls into a string of Unicode triangles:
/// ▲ = 3, ▽ = not 3
fn render_rolls(rolls: &[bool; DICE_COUNT as usize]) -> String {
    rolls
        .iter()
        .map(|&r| if r { '▲' } else { '▽' })
        .intersperse(' ')
        .collect()
}

/// Typical Triangle Agency roll.
pub fn roll() -> RollOutcome {
    let rolls = roll_6d4();
    let count = count_successes(&rolls);
    let result = interpret_roll(count);
    let rendered = render_rolls(&rolls);
    let chaos = result.chaos();
    RollOutcome {
        result,
        rendered,
        chaos,
    }
}
