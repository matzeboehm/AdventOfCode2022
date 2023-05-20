use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn task_1(){
    let file = File::open("src/days/input/day_01.txt").unwrap();
    let lines = BufReader::new(file).lines();


    let mut temp: i32 = 0;
    let mut output: i32 = 0;
    let mut max_user: i32 = 0;
    let mut current_elf: i32 = 1;
    for line in lines {
        let value: String = line.unwrap();
        
        if value.is_empty() {
            if temp > output{
                output = temp;
                max_user = current_elf;
            }
            temp = 0;
            current_elf += 1;
        } else {
            let converted_value: i32 =  value.parse().unwrap();
            temp += converted_value;
        }
    }

    println!("Output {}, Max User {}", output, max_user);
}
    