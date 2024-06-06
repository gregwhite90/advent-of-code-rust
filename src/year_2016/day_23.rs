#[cfg(test)]
use crate::utils::Day;
#[cfg(test)]
const DAY: Day = crate::utils::Day { year: 2016, day: 23 };

pub mod part_one {
    use std::collections::HashMap;

    use crate::{utils::solution::{Answer, Solution}, year_2016::utils::assembunny_computer::AssembunnyComputer};

    #[derive(Debug)]
    pub struct Soln {
        assembunny_computer: AssembunnyComputer,
    }

    impl Solution for Soln {
        fn solve(&mut self, filename: &str) -> Answer {
            self.assembunny_computer.parse_input_file(filename);
            self.assembunny_computer.execute_all();
            Answer::I64(self.assembunny_computer.register_value('a'))
        }
    }

    impl Default for Soln {
        fn default() -> Self {
            Self {
                assembunny_computer: AssembunnyComputer::with_registers(HashMap::from([('a', 7)])),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use test_case::test_case;
        use crate::utils::{test_utils, solution::Answer};
        use super::*;
        use super::super::DAY;

        #[test_case(1, Answer::I64(3); "example_1")]
        fn examples_are_correct(example_key: u8, answer: Answer) {
            test_utils::check_example_case(
                &mut Soln::default(),
                example_key,
                answer,
                &DAY,
            );
        }
    }    
}

/// This part requires a decompilation approach. This is the input instructions:
/// 
/// cpy a b
/// dec b       This below loop is calculating `a`!
/// cpy a d  <------------------------------------------------------------------------------------------------------------------+
/// cpy 0 a                                                                                                                     |
/// cpy b c  <----------------------+                                                                                           |
/// inc a    <--+                   |                                                                                           |
/// dec c       | Loops `c` times   | Loops `d` times                                                                           |
/// jnz c -2 ---+                   |                                                                                           |
/// dec d                           |                                                                                           |
/// jnz d -5 -----------------------+ After these loops, `a` = init `a` * (init `a` - 1), `b` = init `a` - 1, `c` = `d` = 0     |
/// dec b                                                                                                                       |
/// cpy b c                                                                                                                     |
/// cpy c d                                                                                                                     |
/// dec d    <--+ Starting this loop, `c` = `b` = `d` = init `a` - 2                                                            |
/// inc c       | Loops `d` times                                                                                               |
/// jnz d -2 ---+ After this loop, `a` = init `a` * (init `a - 1), `b` = init `a` - 2, `c` = 2 * (init `a` - 2), `d` = 0        |
/// tgl c                                                                                                                       |
/// cpy -16 c                                                                                                                   |
/// jnz 1 c  Until this instruction is toggled, loops back to ------------------------------------------------------------------+
/// cpy 85 c
/// jnz 76 d <----------------------------------------------------------+
/// inc a    <--+                                                       |
/// inc d       | Loops forever unless this instruction is toggled?     |
/// jnz d -2 ---+                                                       |
/// inc c                                                               | Loops forever unless this instruction is toggled?
/// jnz c -5 -----------------------------------------------------------+ Once all the necessary instructions have been toggled,
///                                                                       this set of loops will increment `a` by 85 * 76 = 6_460
/// 
/// The solution (that would also work for part one) is `a`! + 6_460
pub mod part_two {
    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default)]
    pub struct Soln {}

    fn factorial(num: u64) -> u64 {
        if num == 1 { 1 }
        else { num * factorial(num - 1) }
    }

    impl Solution for Soln {
        fn solve(&mut self, _filename: &str) -> Answer {
            Answer::U64(factorial(12) + 85 * 76)
        }
    }
}