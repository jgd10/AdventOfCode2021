pub fn get_input_as_lines(input: &str) -> Vec<&'static str>{
    let mut data: Vec<&str> = Vec::new();
    for line in input.lines() {
        data.push(line)
    }
    data
}


pub fn get_input_as_chars(input: &str) -> Vec<Vec<char>>{
    let mut char_vec: Vec<char>;
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        char_vec = line.chars().collect();
        data.push(char_vec)
    }
    data
}