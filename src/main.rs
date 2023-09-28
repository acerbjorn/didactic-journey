use std::{fmt::Display, fs::File, io::Read, u8};

use serde::{Deserialize, Serialize};

fn main() {
    let mut sudoku_board = SudokuBoard::new();

    sudoku_board.to_file(File::open("board.json").unwrap());

    sudoku_board = SudokuBoard::from_file(File::open("board.json").unwrap());
    print!("{}", sudoku_board);
}

//TODO If Sudokuboard is to contain an Option<u8>, then we must wrap the number in a newtype which must implement serialize and deserialize on it's own.
#[derive(Serialize, Deserialize, Debug)]
struct SudokuBoard([[u8; 9]; 9]);

impl SudokuBoard {
    fn new() -> SudokuBoard {
        SudokuBoard([[0; 9]; 9])
    }
    fn from_file(mut file: File) -> SudokuBoard {
        let mut file_contents = String::new();
        let file_read_result = file.read_to_string(&mut file_contents);
        match file_read_result {
            Ok(val) => ,
            Err(E) => println!(),
        }
        serde_json::from_str(&file_contents).unwrap()
    }
    fn to_file(&self, mut file: File) {
        let serialized_sudoku_board = serde_json::to_string(&self).unwrap();
        serde_json::to_writer(&file, &serialized_sudoku_board).unwrap();
    }
}

impl Display for SudokuBoard {
    //TODO Refactor for readability
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "┌───┬───┬───┐\n")?;
        for (i, line) in self.0.iter().enumerate() {
            write!(f, "│")?;
            for (j, entry) in line.iter().enumerate() {
                if entry == &0u8 {
                    write!(f, "░")?;
                } else {
                    write!(f, "{}", entry)?;
                }
                if j % 3 == 2 {
                    write!(f, "│")?;
                }; //midline
            }
            write! {f,"\n"}?;
            if i == 8 {
                write!(f, "└───┴───┴───┘\n")?;
            }
            //endline
            else if i % 3 == 2 {
                write!(f, "├───┼───┼───┤\n")?;
            }; //midline
        }
        Ok(())
    }
}
