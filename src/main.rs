
mod jhdl;
use jhdl::Board;


fn get_string() -> String {
	let mut input_text = String::new();
	std::io::stdin().read_line(&mut input_text).unwrap();
	input_text
}

fn get_int() -> i32 {
	get_string().trim().parse::<i32>().unwrap_or(0)
}

fn get_char() -> u8 {
	*get_string().as_bytes().get(0).unwrap_or(&0)
}

fn get_int_bits() -> Box<[bool]> {
	let n = get_int();
	(0..16).map(|i| (n >> i) & 1 > 0).collect()
}

fn int_from_bits(bits: &[Option<bool>]) -> Option<i16> {
	let mut n = 0;
	for i in 0..bits.len() {
		if bits[i]? {
			n += 1 << i;
		}
	}
	Some(n)
}




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
