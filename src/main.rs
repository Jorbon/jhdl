
mod jhdl;
use jhdl::Board;

fn main() {
	
	let mut board = Board::new(3);
	
	let a = board.and(0, 1);
	let a2 = board.and(a, 2);
	
	println!("{:?}", board.run(&[true, true, true]).unwrap()[a2]);
	
}
