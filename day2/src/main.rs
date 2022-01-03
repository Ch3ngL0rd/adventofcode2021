use itertools::Itertools;

const FILENAME : &str = "input.txt";
fn main() {
    let contents = std::fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");
    // had to remove the last line for the parse to work correctly
    let data : Vec<_> = contents.split("\n").collect();
    let mut forward : i32 = 0;
    let mut depth : i32 = 0;
    for line in data.clone() {
        let mut split = line.split(' ');
        match split.next().unwrap() {
            "forward" => forward += split.next().unwrap()
                                    .parse::<i32>().unwrap(),
            "down" => depth += split.next().unwrap()
                            .parse::<i32>().unwrap(),
            "up" => depth -= split.next().unwrap()
                            .parse::<i32>().unwrap(),
            _ => println!("Error parsing"),
        }
    }
    println!("#1 Forward : {}, Depth : {}, for a total of {}.",forward,depth,forward*depth);

    // using itertools
    let mut split : (&str,i32);
    let mut aim : i32 = 0;
    // reset old values
    forward = 0;
    depth = 0;
    for line in data {
        if let Some((x,y)) = line.split(' ').collect_tuple() {
            split = (x,y.parse::<i32>().unwrap());
        } else {
            panic!();
        }
        match split.0 {
            "down" => {
                aim += split.1;
            }
            "up" => {
                aim -= split.1;
            }
            "forward" => {
                forward += split.1;
                depth += aim * split.1;
            }
            _ => {
                println!("error!");
                panic!();
            }
        }
    }
    println!("#2 Forward : {}, Depth : {}, for a total of {}.",forward,depth,forward*depth);
}