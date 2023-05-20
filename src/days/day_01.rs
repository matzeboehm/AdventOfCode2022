use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn task_1(){
    let file: File = File::open("src/days/input/day_01.txt").unwrap();
    let lines = BufReader::new(file).lines();


    let mut elf_calories: i32 = 0;
    let mut vec: Vec<i32> = Vec::new();
    for line in lines {
        let value: String = line.unwrap();
        
        if value.is_empty() {
            vec.push(elf_calories);
            elf_calories = 0;
        } else {
            let converted_value: i32 =  value.parse().unwrap();
            elf_calories += converted_value;
        }
    }

    vec.sort();

    let last_three: &[i32] = &vec[vec.len() - 3 .. vec.len()];
    println!("{:?} ", last_three);

    let sum_last_three: i32 = last_three.iter().sum();
    println!("{sum_last_three}")
}
    