
struct Coord {
    x: usize,
    y: usize
}

struct Coord2 {
    x: usize,
    y: usize,
    aim: usize,
}

enum Direction {
    Forward,
    Down,
    Up,
    None,
}


impl Direction {
    fn from_str(input: &str) -> Direction {
        match input {
            "forward"  => Direction::Forward,
            "up"  => Direction::Up,
            "down"  => Direction::Down,
            _ => Direction::None,
        }
    }

}


fn advance_coord(coord: Coord, direction: Direction, amount: usize) -> Coord {
    match direction {
        Direction::Down => {return Coord {x: coord.x, y: coord.y+amount}},
        Direction::Forward => {return Coord {x: coord.x+amount, y: coord.y}},
        Direction::Up => {return Coord {x: coord.x, y: coord.y-amount}},
        Direction::None => {return Coord {x: coord.x, y: coord.y}},
    }
}


fn advance_coord2(coord: Coord2, direction: Direction, amount: usize) -> Coord2 {
    match direction {
        Direction::Down => {return Coord2 {x: coord.x, y: coord.y, aim: coord.aim+amount}},
        Direction::Forward => {return Coord2 {x: coord.x+amount, y: coord.y+amount*coord.aim, aim: coord.aim}},
        Direction::Up => {return Coord2 {x: coord.x, y: coord.y, aim: coord.aim-amount}},
        Direction::None => {return Coord2 {x: coord.x, y: coord.y, aim: coord.aim}},
    }
}


fn magnitude(coord: Coord) -> usize{
    return coord.x*coord.y
}

fn magnitude2(coord: Coord2) -> usize{
    return coord.x*coord.y
}


fn main() {
    println!("{}",part1());
    println!("{}",part2());
}


fn part1() -> usize{
    let input: &str = include_str!("../input.txt");
    let mut coord: Coord = Coord { x: 0, y: 0 };
    let mut value: String;
    for line in input.lines() {
        value = line.parse().unwrap();
        let collection: Vec<&str> = value.split(" ").collect();
        coord = advance_coord(coord, Direction::from_str(collection[0]), collection[1].parse().unwrap())

    }
    return magnitude(coord)
}

fn part2() -> usize{
    let input: &str = include_str!("../input.txt");
    let mut coord: Coord2 = Coord2 { x: 0, y: 0, aim: 0};
    let mut value: String;
    for line in input.lines() {
        value = line.parse().unwrap();
        let collection: Vec<&str> = value.split(" ").collect();
        coord = advance_coord2(coord, Direction::from_str(collection[0]), collection[1].parse().unwrap())

    }
    return magnitude2(coord)
}
