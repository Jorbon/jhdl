
mod parts;
use parts::{Part, Pin};

fn user_interface(p: &Part) {
	
}

fn main() {
	
	let mut num_pins = 0;
	
	let a = Part::and(&Pin::LocalInput(0), &Pin::LocalInput(1), &mut num_pins);
	
	let p = Part::Chip(parts::Chip::new(&[a], &[Pin::LocalInput(0), Pin::LocalInput(1)], &mut 0));
	
}
