

pub fn get_string() -> String {
	let mut input_text = String::new();
	std::io::stdin().read_line(&mut input_text).unwrap();
	input_text
}

pub fn get_int() -> i32 {
	get_string().trim().parse::<i32>().unwrap_or(0)
}

pub fn get_char() -> u8 {
	*get_string().as_bytes().get(0).unwrap_or(&0)
}

pub fn get_int_bits() -> Box<[bool]> {
	let n = get_int();
	(0..16).map(|i| (n >> i) & 1 > 0).collect()
}

pub fn int_from_bits(bits: &[Option<bool>]) -> Option<i16> {
	let mut n = 0;
	for i in 0..bits.len() {
		if bits[i]? {
			n += 1 << i;
		}
	}
	Some(n)
}


