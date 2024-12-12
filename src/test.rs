#[cfg(test)] use crate::core::Board;
#[cfg(test)] use crate::io::{get_char, get_int_bits, int_from_bits};


#[test]
fn main() {
	let mut adder = Board::new(3);
	
	let s1 = adder.xor(0, 1);
	let c1 = adder.and(0, 1);
	let s = adder.xor(s1, 2);
	let c2 = adder.and(s1, 2);
	let c = adder.or(c1, c2);
	
	let mut full_adder = Board::new(33);
	
	let xors: Box<[usize]> = (16..32).map(|i| full_adder.xor(i, 32)).collect();
	
	let mut p = vec![full_adder.chip(adder.clone(), &[0, xors[0], 32], &[s, c])];
	for i in 1..16 {
		p.push(full_adder.chip(adder.clone(), &[i, xors[i], p.last().unwrap()[1]], &[s, c]));
	}
	
	let outputs: Vec<usize> = p.iter().map(|x| x[0]).chain([full_adder.xor(p[14][1], p[15][1])]).collect();
	let output_bits = full_adder.run(&[get_int_bits(), get_int_bits(), Box::new([get_char() == b'-'])].concat(), &outputs).unwrap();
	
	println!("{}{}", int_from_bits(&output_bits[..16]).unwrap(), if let Some(true) = output_bits[16] {" (overflow)"} else {""});
	
}
