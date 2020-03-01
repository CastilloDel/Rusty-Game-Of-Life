mod game_of_life;
use game_of_life::grid::Grid;
use std::{io, time, thread};

const MIN_ROWS: usize = 3;
const MIN_COLS: usize = 3;
const MAX_ROWS: usize = 1000;
const MAX_COLS: usize = 1000;

fn main() {
   println!("Number of rows: ");
   let n_rows = read_number_limits(MIN_ROWS, MAX_ROWS);

   println!("Number of columns: ");
   let n_columns = read_number_limits(MIN_COLS, MAX_COLS);

   let mut game = Grid::new(n_rows, n_columns); 
   println!("Enter the number of alive cells:");
   let n_cells = read_number_limits(0, std::usize::MAX);
   for _i in 0..n_cells {
      print!("\x1B[2J");
      println!("{}", game);
      println!("Enter the row number:");
      let row_pos = read_number_limits(1, n_rows);
      println!("Enter the column number:");
      let col_pos = read_number_limits(1, n_columns);
      game.change_cell_state(row_pos - 1, col_pos - 1);
   }
   loop {
      print!("\x1B[2J");
      println!("{}", game);
      thread::sleep(time::Duration::from_millis(400));
      game.generate_new_gen();
   }
}

fn read_number_limits(lower_limit: usize, upper_limit: usize) -> usize {
   loop {
      let mut number = String::new();
      io::stdin().read_line(&mut number)
         .expect("Couldn't read");
      let number: usize = match number.trim().parse() {
         Ok(num) if num > upper_limit => {
            println!("Please enter a number lesser than or equal to {}", upper_limit);
            continue;
         },
         Ok(num) if num < lower_limit => {
            println!("Please enter a number greater than or equal to {}", lower_limit);
            continue;
         },
         Ok(num) => num,
         Err(_) => {
            println!("Please enter a positive number");
            continue;
         },
      };
      break number
   }
}