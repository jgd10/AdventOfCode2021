fn main() {
    part1();
    part2();
}

struct Rates {
    gamma: usize,
    epsilon: usize,
}

struct Counters {
    c0: Vec<i32>,
    c1: Vec<i32>,
}

struct Numbers {
    ones: Vec<Vec<char>>,
    zeros: Vec<Vec<char>>,
}

fn calc_rates(counters1: Vec<i32>, counters0: Vec<i32>) -> Rates {
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
    for (c1, c0) in counters1.iter().zip(counters0.iter()) {
        if c1 > c0 {
            gamma.push('1');
            epsilon.push('0');
        }
        else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let return_gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let return_epsilon = usize::from_str_radix(&epsilon, 2).unwrap();
    return Rates { gamma: return_gamma, epsilon: return_epsilon }
}

fn count_bits(data: Vec<Vec<char>>) -> Counters {
    let mut counters_1 = vec![0; data[0].len()];
    let mut counters_0 = vec![0; data[0].len()];
    for row in data {
        for (i, c) in row.iter().enumerate() {
            match c {
                '1' => counters_1[i] += 1,
                '0' => counters_0[i] += 1,
                _ => {},
            }
        }
    }
    return Counters{c1: counters_1, c0: counters_0}

}

fn split_by_integer_at_index(data: Vec<Vec<char>>, index: usize) -> Numbers {
    let mut ones: Vec<Vec<char>> = Vec::new();
    let mut zeros: Vec<Vec<char>> = Vec::new();
    for row in data {
        match row[index] {
            '1' => ones.push(row),
            '0' => zeros.push(row),
            _ => {},
        }
    }
    return Numbers {ones, zeros};
} 

fn parse_input() -> Vec<Vec<char>>{
    let input: &str = include_str!("../input.txt");
    let mut char_vec: Vec<char>;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_vec = line.chars().collect();
        data.push(char_vec)
    }
    return data;
}

fn calc_oxygen_co2_rates(data: Vec<Vec<char>>) -> (usize, usize) {
    let mut oxy_nums: Numbers;
    let mut co2_nums: Numbers;
    let mut oxy_data: Vec<Vec<char>> = data.clone();
    let mut co2_data: Vec<Vec<char>> = data.clone();
    let mut index: usize = 0;
    while oxy_data.len() > 1 || co2_data.len() > 1 {
        oxy_nums = split_by_integer_at_index(oxy_data.clone(), index);
        co2_nums = split_by_integer_at_index(co2_data.clone(), index); 
        if oxy_data.len() > 1{          
            if oxy_nums.ones.len() >= oxy_nums.zeros.len() {
                oxy_data = oxy_nums.ones;
            }
            else {
                oxy_data = oxy_nums.zeros;
            }
        }
        if co2_data.len() > 1{
            if co2_nums.ones.len() < co2_nums.zeros.len() {
                co2_data = co2_nums.ones;
            }
            else {
                co2_data = co2_nums.zeros;
            }
        }
    
        index += 1;
    }
    return (usize::from_str_radix(&oxy_data[0].iter().collect::<String>(),
     2).unwrap(), usize::from_str_radix(&co2_data[0].iter().collect::<String>(), 2).unwrap());

}


fn part1() {
    let rates: Rates;
    let counters: Counters;
    let data: Vec<Vec<char>>;
    data = parse_input();
    counters = count_bits(data);
    rates = calc_rates(counters.c1, counters.c0);
    println!("{}", rates.gamma*rates.epsilon);
}

fn part2() {
    let data: Vec<Vec<char>>;
    let oxy_rate: usize;
    let co2_rate: usize;
    data = parse_input();
    (oxy_rate, co2_rate) = calc_oxygen_co2_rates(data);
    println!("{}", oxy_rate*co2_rate);
}

