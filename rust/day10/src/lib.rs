extern crate core;

use std::ops::Add;

struct Instruction {
    command: String,
    value: i32,
}


pub fn process_part1(input: &str) -> String {
    let signallevels: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let instructions: Vec<Instruction> = input_to_instructions(input);
    let cycles: Vec<i32> = instructions_to_cycles(instructions);
    let signalintensities: Vec<i32> = cycles.iter()
        .scan(1, |acc, signal| {
            *acc = *acc + signal;
            Some(*acc)
        })
        .map(|x| x.clone())
        .collect();


    let result: i32 = signallevels.iter()
        .map(|level| {
            println!("{}", level - 2);
            println!("{}", signalintensities.get((level - 2) as usize).unwrap_or(&0) * level);
            signalintensities.get((level - 2) as usize).unwrap_or(&0) * level
        })
        .sum();

    return result.to_string();
}

fn instructions_to_cycles(instructions: Vec<Instruction>) -> Vec<i32> {
    let mut cycles: Vec<i32> = vec![];

    instructions.iter()
        .for_each(|instruction| {
            if instruction.command.eq("addx")
            { cycles.push(0) }
            cycles.push(instruction.value);
        });

    return cycles;
}

fn input_to_instructions(input: &str) -> Vec<Instruction> {
    return input
        .lines()
        .map(|line| line_to_instruction(line))
        .collect();
}

fn line_to_instruction(line: &str) -> Instruction {
    if line.eq("noop") {
        return Instruction { command: "noop".to_string(), value: 0 };
    } else if line.starts_with("addx") {
        return Instruction { command: "addx".to_string(), value: line.split(' ').last().unwrap().parse::<i32>().unwrap() };
    } else {
        panic!("Couldn't parse instruction");
    }
}

pub fn process_part2(input: &str) -> String {
    let signallevels: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let instructions: Vec<Instruction> = input_to_instructions(input);
    let cycles: Vec<i32> = instructions_to_cycles(instructions);
    let signalintensities: Vec<i32> = cycles.iter()
        .scan(1, |acc, signal| {
            *acc = *acc + signal;
            Some(*acc)
        })
        .map(|x| x.clone())
        .collect();


    let mut pixels: String = "".to_string();

    for i in 0..cycles.len() {
        if i % 40 == 0 {
            pixels = pixels.clone().add("\n");
        }
        pixels = pixels.clone().add(&parse_to_pixel(*cycles.get(i).unwrap()));
    }

    return pixels;
}

fn parse_to_pixel(input: i32) -> String {
    if (input == 0) {
        return "-".parse().unwrap()
    }
    else {
     return "#".parse().unwrap();}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_input_then() {}
}