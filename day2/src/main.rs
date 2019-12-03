use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input_text = String::new();
    input_file.read_to_string(&mut input_text).unwrap();

    let instructions: Vec<&str> = input_text.split(",").map(|value| value.trim()).collect();
    let initial_memory: Vec<i32> = instructions.into_iter().map(|value| value.parse().unwrap()).collect();
    let mut instructions = initial_memory.clone();
    instructions[1] = 12;
    instructions[2] = 2;
    println!("Program output (part one): {}", execute_program(instructions));

    for i in 0..100 {
        for j in 0..100 {
            let mut program = initial_memory.clone();
            program[1] = i;
            program[2] = j;
            let output = execute_program(program);
            if output == 19690720 {
                println!("Raw inputs are: {}, {}", i, j);
                println!("Progam input: {}", 100 * i + j);
                return;
            }
        }
    }

    println!("Failed to find correct input");
}

fn execute_program(mut program: Vec<i32>) -> i32 {
    let mut read_position = 0;
    while read_position < program.len() {
        read_position += execute_instruction(&mut program, read_position);
    }
    program[0]
}

fn execute_instruction(memory: &mut Vec<i32>, instruction_pointer: usize) -> usize {
    match memory[instruction_pointer] {
        // Addition
        1 => {
            let input_pos_left = memory[instruction_pointer + 1] as usize;
            let input_pos_right = memory[instruction_pointer + 2] as usize;
            let output_pos = memory[instruction_pointer + 3] as usize;
            memory[output_pos] = memory[input_pos_left] + memory[input_pos_right];

            4
        }
        // Multiplication
        2 => {
            let input_pos_left = memory[instruction_pointer + 1] as usize;
            let input_pos_right = memory[instruction_pointer + 2] as usize;
            let output_pos = memory[instruction_pointer + 3] as usize;
            memory[output_pos] = memory[input_pos_left] * memory[input_pos_right];

            4
        }
        // Program halt, return memory size so we advance the instruction pointer past size of memory
        99 => memory.len(),
        _ => { panic!("Unrecognized instruction") }
    }
}
