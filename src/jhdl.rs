
#[derive(Debug)]
pub struct Board {
	pub input_pins: usize,
	pub pin_count: usize,
	pub parts: Vec<Part>
}

#[derive(Debug)]
pub enum Part {
	Gate(Gate),
	Chip(Chip)
}

#[derive(Debug, Clone, Copy)]
pub enum Gate {
	Source(usize, bool),
	Not(usize, usize),
	And(usize, usize, usize),
	Or(usize, usize, usize)
}

#[derive(Debug)]
pub struct Chip {
	pub input_map: Vec<(usize, usize)>,
	pub output_map: Vec<(usize, usize)>,
	pub parts: Vec<Part>,
	pub pin_count: usize
}


impl Board {
	pub fn new(input_pins: usize) -> Self {
		Self {
			input_pins,
			pin_count: input_pins,
			parts: vec![]
		}
	}
	pub fn source(&mut self, value: bool) -> usize {
		self.parts.push(Part::Gate(Gate::Source(self.pin_count, value)));
		self.pin_count += 1;
		self.pin_count - 1
	}
	pub fn not(&mut self, input: usize) -> usize {
		self.parts.push(Part::Gate(Gate::Not(self.pin_count, input)));
		self.pin_count += 1;
		self.pin_count - 1
	}
	pub fn and(&mut self, input1: usize, input2: usize) -> usize {
		self.parts.push(Part::Gate(Gate::And(self.pin_count, input1, input2)));
		self.pin_count += 1;
		self.pin_count - 1
	}
	pub fn or(&mut self, input1: usize, input2: usize) -> usize {
		self.parts.push(Part::Gate(Gate::Or(self.pin_count, input1, input2)));
		self.pin_count += 1;
		self.pin_count - 1
	}
	pub fn chip(&mut self, board: Board, inputs: &[usize], outputs: &[usize]) -> Vec<usize> {
		self.parts.push(Part::Chip(Chip {
			input_map: (0..board.input_pins).map(|i| (i, inputs[i])).collect(),
			output_map: (0..outputs.len()).map(|i| (outputs[i], self.pin_count + i)).collect(),
			parts: board.parts,
			pin_count: board.pin_count
		}));
		self.pin_count += outputs.len();
		((self.pin_count - outputs.len())..self.pin_count).collect()
	}
	
	pub fn run(&self, input: &[bool]) -> Result<Vec<Option<bool>>, ()> {
		let mut pins = vec![None; self.pin_count];
		for i in 0..self.input_pins {
			pins[i] = Some(input[i]);
		}
		for part in &self.parts {
			match part {
				Part::Gate(gate) => gate.update(&mut pins)?,
				Part::Chip(chip) => chip.update(&mut pins)?
			}
		}
		Ok(pins)
	}
}



impl Chip {
	pub fn update(&self, pins: &mut Vec<Option<bool>>) -> Result<(), ()> {
		let mut cpins = vec![None; self.pin_count];
		
		for (_int, ext) in &self.output_map {
			if let Some(_) = pins[*ext] {
				Err(())?
			}
		}
		for (int, ext) in &self.input_map {
			pins[*ext].ok_or(())?;
			cpins[*int] = pins[*ext];
		}
		for part in &self.parts {
			match part {
				Part::Gate(gate) => gate.update(&mut cpins)?,
				Part::Chip(chip) => chip.update(&mut cpins)?
			}
		}
		for (int, ext) in &self.output_map {
			pins[*ext] = cpins[*int];
		}
		Ok(())
	}
}



impl Gate {
	pub fn update(&self, pins: &mut Vec<Option<bool>>) -> Result<(), ()> {
		match self {
			Gate::Source(o, value) => match pins[*o] {
				Some(_) => Err(()),
				None => {
					pins[*o] = Some(*value);
					Ok(())
				}
			}
			Gate::Not(o, i) => match pins[*i] {
				Some(b) => match pins[*o] {
					Some(_) => Err(()),
					None => {
						pins[*o] = Some(!b);
						Ok(())
					}
				}
				None => Err(())
			}
			Gate::And(o, i1, i2) => match pins[*i1] {
				Some(b1) => match pins[*i2] {
					Some(b2) => match pins[*o] {
						Some(_) => Err(()),
						None => {
							pins[*o] = Some(b1 && b2);
							Ok(())
						}
					}
					None => Err(())
				}
				None => Err(())
			}
			Gate::Or(o, i1, i2) => match pins[*i1] {
				Some(b1) => match pins[*i2] {
					Some(b2) => match pins[*o] {
						Some(_) => Err(()),
						None => {
							pins[*o] = Some(b1 || b2);
							Ok(())
						}
					}
					None => Err(())
				}
				None => Err(())
			}
		}
	}
}




