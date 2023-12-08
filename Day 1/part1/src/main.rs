
fn main(){
    part1();
    part2();
} 

fn part1() {
    let input: &str = include_str!("../../input.txt");
    let mut value: i32;
    let mut previous_value: i32 = 0;
    let mut counter: i32 = 0;
    for line in input.lines() {
        value = line.parse().unwrap();
        if value > previous_value {
            counter = counter + 1;
        }
        previous_value = value;
    }
    println!("{}", counter-1)
}

fn create_input(input: &str) -> Vec<i32>{
    let mut numbers: Vec<i32> = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    }
    return numbers;
}

fn part2() {
    let input: &str = include_str!("../../input.txt");
    let depths: Vec<i32> = create_input(input);
    let mut counter: i32 = 0;
    let max_index: usize = depths.len(); 
    for i in 3..max_index {
        if there_is_a_rise(i, depths.clone()) {
            counter = counter + 1
        }
    }
    println!("{}", counter)
}

fn there_is_a_rise(index: usize, depths: Vec<i32>) -> bool{
    let back_total: i32;
    let forward_total: i32;
    let back_start: usize = index - 3;
    let forward_start: usize = index - 2;
    let forward_end: usize = index + 1;
    let back_slice: &[i32] = &depths[back_start..index];
    let forward_slice: &[i32] = &depths[forward_start..forward_end];
    back_total = back_slice.iter().sum();
    forward_total = forward_slice.iter().sum();
    // println!("{:?}, {:?}, {}, {}", back_slice, forward_slice, back_total, forward_total);
    if forward_total > back_total {
        return true;
    }
    else {
        return false
    }
}
