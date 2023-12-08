mod aoc_parser;
use aoc_parser::get_input_as_lines;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Square {
    Mark,
    UnMark,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Board {
    marks: Vec<Vec<Square>>,
    numbers: Vec<Vec<usize>>,
}

impl Board {
    fn mark_number(&mut self, number: &usize) {
        for (i, row) in self.numbers.iter().enumerate() {
            for (j, column) in row.iter().enumerate(){
                if column == number {
                    self.marks[i][j] = Square::Mark;
                }
            }
        }
    }
    fn has_bingo(&self) -> bool{
        let mut columns: Vec<Vec<&Square>> = Vec::new();
        for _i in 0..5{
            columns.push(Vec::new());
        }
        for row in &self.marks {
            let set: HashSet<&Square> = row.iter().collect();
            if set.contains(&Square::Mark) && set.len() == 1{
                return true;
            } 
            for (j, val) in row.iter().enumerate() {
                columns[j].push(val);
            }    
       }
       for column in columns.clone() {
            let column_set: HashSet<&Square> = column.into_iter().collect();
            if column_set.contains(&Square::Mark) && column_set.len() == 1{
                return true;
            } 
        }
        false
    }
    fn sum_unmarked_nums(&self) -> usize {
        let mut total: usize = 0;
        for (row, row_marks) in self.numbers.iter().zip(self.marks.clone()) {
            for (number, mark) in row.iter().zip(row_marks){
                match mark{
                    Square::Mark => (),
                    Square::UnMark => total += number,
                }
            }
        }
        total
    }
}

fn get_board_from_lines(mut lines: Vec<&str>) -> Board{
    lines.retain(|&x| !x.is_empty() );
    assert_eq!(lines.len(), 5);
    let mut marks: Vec<Vec<Square>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    for (j, line) in lines.iter().enumerate() {
        numbers.push(line.split_ascii_whitespace().into_iter().map(|x| x.parse::<usize>().unwrap()).collect());
        for _i in 0..5 {
            marks[j].push(Square::UnMark);
        }
    }
    Board { marks: marks, numbers: numbers }
}

fn parse_input() -> (Vec<usize>, Vec<Board>) {
    let data = get_input_as_lines(include_str!("../input.txt"));
    let numbers: Vec<usize> = data[0].split(',').map(|x: &str| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut boards: Vec<Board> = Vec::new();
    let board_slices: Vec<Vec<&str>> = data[2..].chunks(6).map(|x| x.to_vec()).collect();
    for board_slice in board_slices {
        boards.push(get_board_from_lines(board_slice))
    }
    (numbers, boards)
}

#[test]
pub fn test_board_all_marks() {
    let board: Board = Board { 
        marks: vec![
            vec![Square::Mark,Square::Mark,Square::Mark,Square::Mark,Square::Mark],
            vec![Square::Mark,Square::Mark,Square::Mark,Square::Mark,Square::Mark],
            vec![Square::Mark,Square::Mark,Square::Mark,Square::Mark,Square::Mark],
            vec![Square::Mark,Square::Mark,Square::Mark,Square::Mark,Square::Mark],
            vec![Square::Mark,Square::Mark,Square::Mark,Square::Mark,Square::Mark]
        ], 
        numbers: vec![
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1]
        ] 
    };
    assert!(board.has_bingo());
}

#[test]
pub fn test_board_row_marks() {
    let board: Board = Board { 
        marks: vec![
            vec![Square::Mark,Square::Mark,Square::Mark,Square::Mark,Square::Mark],
            vec![Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark]
        ], 
        numbers: vec![
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1]
        ] 
    };
    assert!(board.has_bingo());
}

#[test]
pub fn test_board_col_marks() {
    let board: Board = Board { 
        marks: vec![
            vec![Square::Mark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::Mark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::Mark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::Mark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark],
            vec![Square::Mark,Square::UnMark,Square::UnMark,Square::UnMark,Square::UnMark]
        ], 
        numbers: vec![
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1],
            vec![1,1,1,1,1]
        ] 
    };
    assert!(board.has_bingo());
}

fn part1(){
    let mut data: (Vec<usize>, Vec<Board>) = parse_input();
    let mut no_bingo: bool = true;
    let mut index: usize = 0;
    let mut final_number: usize = 0;
    let mut winning_sum: usize = 0;
    while no_bingo  && index < data.0.len(){
        for board in &mut data.1 {
            board.mark_number(&data.0[index]);
            //dbg!(board.marks.clone());
            if board.has_bingo() {
                no_bingo = false;
                final_number = data.0[index].clone();
                winning_sum = board.sum_unmarked_nums();
                break
            }
        }
        index += 1;
    }
    println!("Part 1 Answer: {}", final_number*winning_sum)
}

fn last_board_has_not_won(boards: &Vec<bool>) -> bool {
    !boards.clone().iter().all(|&item| item)
}

fn part2(){
    let mut data: (Vec<usize>, Vec<Board>) = parse_input();
    let mut index: usize = 0;
    let mut final_number: usize = 0;
    let mut winning_sum: usize = 0;
    let mut board_index: usize;
    let mut boards_have_bingo: Vec<bool> = Vec::new();
    for _i in 0..data.1.len(){
        boards_have_bingo.push(false)
    }
    while last_board_has_not_won(&boards_have_bingo)  && index < data.0.len(){
        board_index = 0;
        for board in &mut data.1 {
            board.mark_number(&data.0[index]);
            //dbg!(board.marks.clone());
            if board.has_bingo() {
                boards_have_bingo[board_index] = true;
            }
            if !last_board_has_not_won(&boards_have_bingo){
                final_number = data.0[index].clone();
                winning_sum = board.sum_unmarked_nums();
                break
            }
            board_index += 1
        }
        index += 1;
    }
    println!("Part 2 Answer: {}", final_number*winning_sum)
}

fn main() {
    part1();
    part2();
}
