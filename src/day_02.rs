use crate::intcode;

fn fix_crash(mut memory: Vec<isize>) -> Vec<isize> {
    memory[1] = 12;
    memory[2] = 2;
    memory
}

fn attempt_run(mut memory: Vec<isize>, x: isize, y: isize) -> Vec<isize> {
    memory[1] = x;
    memory[2] = y;
    intcode::run(memory, 0)
}

fn brute_force(memory: Vec<isize>) -> (isize, isize) {
    for x in 0..100 {
        for y in 0..100 {
            let copy = memory.clone();
            let value = *attempt_run(copy, x, y).iter().nth(0).unwrap();
            if value == 19_690_720 {
                return (x, y);
            }
        }
    }
    panic!("Match not found for combination of x and y, Aborting!");
}

pub fn part_1(mut memory: Vec<isize>) -> String {
    memory = fix_crash(memory);
    let result = *intcode::run(memory, 0).iter().nth(0).unwrap();
    format!("Part 1: {}", result).to_string()
}

pub fn part_2(memory: Vec<isize>) -> String {
    let (x, y) = brute_force(memory);
    format!("Part 2: {}", 100 * x + y).to_string()
}

#[cfg(test)]
mod day_02_tests {
    #[test]
    fn part_1() {
        // Moved to intcode module
    }
    #[test]
    fn part_2() {}
}
