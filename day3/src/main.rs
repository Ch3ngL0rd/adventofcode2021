const FILENAME : &str = "input.txt";
fn main() {
    let contents = std::fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");
    println!("Reading file {}",FILENAME);
    // had to remove the last line for the parse to work correctly
    let data : Vec<_> = contents.split("\n").collect();
    let mut bits : Vec<i32> = Vec::with_capacity(data[0].len() as usize);
    for _i in 0.. data[0].len() {
        bits.push(0);
    }
    for line in data.clone() {
        for (index,item) in line.chars().enumerate() {
            if item == '1' {
                bits[index as usize] += 1;
            } else {
                bits[index as usize] -= 1;
            }
        }
    }
    // find power
    let mut gamma : Vec<i32> = Vec::with_capacity(bits.len() as usize);
    let mut epsilon : Vec<i32> = gamma.clone();
    for num in bits {
        if num > 0 {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }
    println!("{:?}",gamma); 
    println!("gamma : {}, episilon : {}",binary_to_decimal(&gamma),binary_to_decimal(&epsilon));
    println!("total is {}", binary_to_decimal(&gamma) * binary_to_decimal(&epsilon));
}

fn binary_to_decimal(input_vector : &Vec<i32>) -> i32 {
    let mut total : i32 = 0;
    for (index,num) in input_vector.iter().rev().enumerate() {
        total += num * 2_i32.pow(index as u32);
    }
    total
}