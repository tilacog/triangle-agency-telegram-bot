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
/// A die is Symbol’s value as variable is void: true if it rolled a 3, Symbol’s
/// value as variable is void: false otherwise.
fn roll_6d4<R>(rng: &mut R) -> [bool; DICE_COUNT as usize]
where
    R: Rng + ?Sized,
{
    std::array::from_fn(|_| rng.random_range(1 ..= 4) == 3)
}

/// Count how many 3’s appeared in the rolls.
fn count_successes(rolls: &[bool; DICE_COUNT as usize]) -> u8 {
    rolls.iter().filter(|&&r| r).count() as u8
}

/// Interpret the count of 3’s as a Symbol’s value as variable is void:
/// RollResult.
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
    rolls.iter().map(|&r| if r { '▲' } else { '▽' }).intersperse(' ').collect()
}

/// Typical Triangle Agency roll.
pub fn roll<R>(rng: &mut R) -> RollOutcome
where
    R: Rng + ?Sized,
{
    let rolls = roll_6d4(rng);
    let count = count_successes(&rolls);
    let result = interpret_roll(count);
    let rendered = render_rolls(&rolls);
    let chaos = result.chaos();
    RollOutcome { result, rendered, chaos }
}

#[cfg(test)]
mod tests {
    use crate::rng::create_rng;

    use super::*;

    #[test]
    fn test_count_successes_zero() {
        let rolls = [false, false, false, false, false, false];
        assert_eq!(count_successes(&rolls), 0);
    }

    #[test]
    fn test_count_successes_some() {
        let rolls = [true, false, true, false, false, true];
        assert_eq!(count_successes(&rolls), 3);
    }

    #[test]
    fn test_count_successes_all() {
        let rolls = [true, true, true, true, true, true];
        assert_eq!(count_successes(&rolls), 6);
    }

    #[test]
    fn test_interpret_roll_failure() {
        let result = interpret_roll(0);
        assert!(matches!(result, RollResult::Failure));
    }

    #[test]
    fn test_interpret_roll_triscendence() {
        let result = interpret_roll(3);
        assert!(matches!(result, RollResult::Triscendence));
    }

    #[test]
    fn test_interpret_roll_success() {
        for count in [1, 2, 4, 5, 6] {
            let result = interpret_roll(count);
            assert!(matches!(result, RollResult::Success(c) if c == count));
        }
    }

    #[test]
    fn test_chaos_failure() {
        let result = RollResult::Failure;
        assert_eq!(result.chaos(), 6);
    }

    #[test]
    fn test_chaos_triscendence() {
        let result = RollResult::Triscendence;
        assert_eq!(result.chaos(), 0);
    }

    #[test]
    fn test_chaos_success() {
        assert_eq!(RollResult::Success(1).chaos(), 5);
        assert_eq!(RollResult::Success(2).chaos(), 4);
        assert_eq!(RollResult::Success(4).chaos(), 2);
        assert_eq!(RollResult::Success(5).chaos(), 1);
        assert_eq!(RollResult::Success(6).chaos(), 0);
    }

    #[test]
    fn test_render_rolls_all_success() {
        let rolls = [true, true, true, true, true, true];
        assert_eq!(render_rolls(&rolls), "▲ ▲ ▲ ▲ ▲ ▲");
    }

    #[test]
    fn test_render_rolls_all_failure() {
        let rolls = [false, false, false, false, false, false];
        assert_eq!(render_rolls(&rolls), "▽ ▽ ▽ ▽ ▽ ▽");
    }

    #[test]
    fn test_render_rolls_mixed() {
        let rolls = [true, false, true, false, true, false];
        assert_eq!(render_rolls(&rolls), "▲ ▽ ▲ ▽ ▲ ▽");
    }

    #[test]
    fn test_roll_outcome_structure() {
        let mut rng = create_rng([0_u8]);
        let outcome = roll(&mut rng);
        // Just verify the outcome has valid structure
        assert!(outcome.chaos <= 6);
        assert!(!outcome.rendered.is_empty());
    }

    #[test]
    fn test_display_result_failure() {
        let result = RollResult::Failure;
        assert_eq!(result.to_string(), "❌");
    }

    #[test]
    fn test_display_result_success() {
        let result = RollResult::Success(2);
        assert_eq!(result.to_string(), "✅");
    }

    #[test]
    fn test_display_result_triscendence() {
        let result = RollResult::Triscendence;
        assert_eq!(result.to_string(), "✨");
    }

    #[test]
    fn test_display_outcome() {
        let outcome = RollOutcome {
            result: RollResult::Success(2),
            rendered: "▲ ▲ ▽ ▽ ▽ ▽".to_string(),
            chaos: 4,
        };
        assert_eq!(outcome.to_string(), "✅  ▲ ▲ ▽ ▽ ▽ ▽\n🌀 4");
    }
}
