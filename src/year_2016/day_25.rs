/// This part requires a decompilation approach. This is the input instructions:
/// 
/// cpy a d
/// cpy 9 c
/// cpy 282 b <-----+
/// inc d     <-+   |
/// dec b       |   | Inner loops 282 times.  
/// jnz b -2  --+   |
/// dec c           | Outer loops 9 times. 
/// jnz c -5  ------+ `d` = init `a` + 282 * 9 after this double loop. 'b', 'c' = 0.
/// cpy d a   <---\_________________________________  Loops back to top instruction if `a == 0`,
/// jnz 0 0   <---/                                 | else loops back to the no-op (jnz 0 0).
/// cpy a b                                     | d a b all equal
/// cpy 0 a                                     |
/// cpy 2 c   <---------------------+ 'outer    |
/// jnz b 2   <-+ Break 'outer      |           |
/// jnz 1 6     | if `b == 0`       |           |
/// dec b       |                   |           |
/// dec c       |                   |           |
/// jnz c -4  --+                   |           |
/// inc a                           |           |
/// jnz 1 -7  ----------------------+           | a = b / 2. c = 2 if b % 2 == 0, 1 if b % 2 == 1.
/// cpy 2 b                                     |
/// jnz c 2   <-+ Break if `c == 0`.             |
/// jnz 1 4     | Will output 1 if `c == 1` to start. This happens when `b == 1` going into an iteration of `outer.                             |
/// dec b       | Will output 0 if `c == 2` to start. This happens when `b == 0` going into an iteration of `outer.                              |
/// dec c       |                               |
/// jnz 1 -4  --+                               |
/// jnz 0 0                                     |
/// out b                                       |
/// jnz a -19   \_________________________________|
/// jnz 1 -21   /
/// 
/// This is a repeated cycle where the initial `a` + 282 * 9 is divided by 2 and the
/// remainder is output. Thinking of the initial `a` + 282 * 9 as a binary representation
/// of an unsigned int, it's effectively outputting the least significant bit
/// and then right-shifting by 1 each iteration. So we need the smallest unsigned int
/// greater than 282 * 9 for which the binary representation is alternating 1s and 0s
/// starting with a 0 in the least significant bit. The binary representation of 282 * 9
/// is `0b100111101010`. So we need `init_a + 282 * 9 = 0b101010101010 = 2_730` and
/// `init_a = 192`.
pub mod part_one {
    use crate::utils::solution::{Answer, Solution};

    #[derive(Debug, Default)]
    pub struct Soln {}

    impl Solution for Soln {
        fn solve(&mut self, _filename: &str) -> Answer {
            Answer::U32(0b101010101010 - 282 * 9)
        }
    }
}