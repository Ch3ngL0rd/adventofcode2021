use std::fs;

const FILENAME : &str = "input.txt";

fn main() {
    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    // had to remove the last line for the parse to work correctly
    let mut data = split.map(|x| x.parse::<i32>().unwrap());
    let vector : Vec<_> = data.clone().collect();

    let mut previous_value = data.next().unwrap();
    let mut total = 0;
    for value in data {
        if value > previous_value {
            total += 1;
        }
        previous_value = value;
    }
    println!("The total measurements that increased were {}.", total);

    let mut prev_sum : i32 = vector[0] + vector[1] + vector[2];
    let mut sum : i32;
    total = 0;
    for index in 1..(vector.len()-2) {
        sum = vector[index] + vector[index+1] + vector[index+2];
        if sum > prev_sum {
            total += 1;
        }
        prev_sum = sum;
    }
    println!("The total amount of sum increases were {}.", total);
}