use std::fs;

pub fn run() -> i32 {
    let filename: &str = "input/day1.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut frequency: i32 = 0;
    for _output in contents
        .lines()
        .map(|line| {
            let (operator, operand) = first_char(line);
            if operator == "+" {
                frequency += operand.parse::<i32>().unwrap();
            } else {
                frequency -= operand.parse::<i32>().unwrap();
            }
            frequency
        }) {
    };
    frequency
}

// This function needs optimising, it takes ages
pub fn run2() -> i32 {
    let filename: &str = "input/day1.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut frequency: i32 = 0;
    let mut frequencies: Vec<i32> = Vec::new();
    let frequency: Option<i32> = loop {
        let mut found: bool = false;
        for output in contents
            .lines()
            .map(|line| {
                let (operator, operand) = first_char(line);
                if operator == "+" {
                    frequency += operand.parse::<i32>().unwrap();
                } else {
                    frequency -= operand.parse::<i32>().unwrap();
                }
                frequency
            }) {
            found = find_match(frequencies.clone(), output);
            frequencies.push(output);
            // This is only so that you can see that it's doing something
            println!("Frequency: {}", output);
            if found == true {
                println!("Doubled Frequency: {}", output);
                break;
            }
        };
        if found == true {
            break frequencies.pop();
        }
    };
    frequency.unwrap()
}

fn first_char(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn find_match(v: Vec<i32>, find: i32) -> bool {
    let r:bool;
    let found: Option<&i32> = v.iter().find(|&&x| x == find);
    match found {
        Some(_c) => r = true,
        None => r = false,
    }
    r
}
