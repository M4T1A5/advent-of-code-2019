use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    // part one
    let mut simple_total_fuel = 0;
    // part two
    let mut total_fuel_with_fuel_weight = 0;
    for line in input.lines() {
        let weight: i32 = line.parse().unwrap();

        // part one
        simple_total_fuel += calc_fuel_requirement(weight);

        // part two
        let mut module_total_fuel_requirement = 0;
        let mut module_fuel_requirement = calc_fuel_requirement(weight);
        module_total_fuel_requirement += module_fuel_requirement;
        while module_fuel_requirement > 0 {
            module_fuel_requirement = calc_fuel_requirement(module_fuel_requirement);
            module_total_fuel_requirement += if module_fuel_requirement > 0 { module_fuel_requirement } else { 0 };
        }
        total_fuel_with_fuel_weight += module_total_fuel_requirement;
    }
    println!("Required fuel: {}", simple_total_fuel);
    println!("Required fuel for part two: {}", total_fuel_with_fuel_weight);
}

fn calc_fuel_requirement(value: i32) -> i32 {
    value / 3 - 2
}

