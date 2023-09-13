
#[derive(Debug)]
pub enum Part {
	Gate(Gate),
	Chip(Chip)
}

#[derive(Debug)]
pub enum Gate {
	Source(usize, bool),
	Not(usize, usize),
	And(usize, usize, usize),
	Or(usize, usize, usize),
	LocalOutput(usize)
}

#[derive(Debug)]
pub struct Chip {
	pub input_map: Vec<(usize, usize)>,
	pub output_map: Vec<(usize, usize)>,
	pub parts: Vec<Part>,
	pub pin_count: usize
}

#[derive(Debug)]
pub enum Pin {
	GateOutput(Gate),
	ChipOutput(Chip, usize),
	LocalInput(usize)
}




impl Gate {
	pub fn source(value: bool, pin_count: &mut usize) -> Gate {
		*pin_count += 1;
		Gate::Source(*pin_count - 1, value)
	}
	pub fn not(input: &Pin, pin_count: &mut usize) -> Gate {
		*pin_count += 1;
		if let Pin::LocalInput(_) = input { *pin_count += 1; }
		Gate::Not(*pin_count - 1, input.location().unwrap())
	}
	pub fn and(input1: &Pin, input2: &Pin, pin_count: &mut usize) -> Gate {
		*pin_count += 1;
		if let Pin::LocalInput(_) = input1 { *pin_count += 1; }
		if let Pin::LocalInput(_) = input2 { *pin_count += 1; }
		Gate::And(*pin_count - 1, input1.location().unwrap(), input2.location().unwrap())
	}
	pub fn or(input1: &Pin, input2: &Pin, pin_count: &mut usize) -> Gate {
		*pin_count += 1;
		if let Pin::LocalInput(_) = input1 { *pin_count += 1; }
		if let Pin::LocalInput(_) = input2 { *pin_count += 1; }
		Gate::Or(*pin_count - 1, input1.location().unwrap(), input2.location().unwrap())
	}
	
	pub fn output_pin(&self) -> Option<usize> {
		match self {
			Gate::Source(p, ..) => Some(*p),
			Gate::Not(p, ..) => Some(*p),
			Gate::And(p, ..) => Some(*p),
			Gate::Or(p, ..) => Some(*p),
			Gate::LocalOutput(_) => None
		}
	}
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
			Gate::LocalOutput(i) => pins[*i].ok_or(())?
		}
	}
}

impl Chip {
	pub fn new(parts: &[Part], inputs: &[Pin], pin_count: &mut usize) -> Chip {
		let mut num_pins = 0;
		for part in parts {
			
		}
		
		Chip {
			input_map: vec![],
			output_map: vec![],
			parts: vec![],
			pin_count: num_pins
		}
	}
	
	pub fn update(&self, pins: &mut Vec<Option<bool>>) -> Result<(), ()> {
		let num_cpins = usize::max(self.input_map.iter().max_by_key(|x| x.0).unwrap().0, self.output_map.iter().max_by_key(|x| x.0).unwrap().0);
		let mut cpins = vec![None; num_cpins];
		
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
			part.update(&mut cpins)?;
		}
		for (int, ext) in &self.output_map {
			pins[*ext] = cpins[*int];
		}
		Ok(())
	}
}


impl Pin {
	pub fn location(&self) -> Option<usize> {
		match self {
			Pin::GateOutput(gate) => gate.output_pin(),
			Pin::ChipOutput(chip, pin) => chip.output_map.get(*pin),
			Pin::LocalInput(pin) => Some(*pin)
		}
	}
}



impl Part {
	pub fn source(value: bool, pin_count: &mut usize) -> Part {
		Part::Gate(Gate::source(value, pin_count))
	}
	pub fn not(input: &Pin, pin_count: &mut usize) -> Part {
		Part::Gate(Gate::not(input, pin_count))
	}
	pub fn and(input1: &Pin, input2: &Pin, pin_count: &mut usize) -> Part {
		Part::Gate(Gate::and(input1, input2, pin_count))
	}
	pub fn or(input1: &Pin, input2: &Pin, pin_count: &mut usize) -> Part {
		Part::Gate(Gate::or(input1, input2, pin_count))
	}
	
	pub fn update(&self, pins: &mut Vec<Option<bool>>) -> Result<(), ()> {
		match self {
			Part::Gate(gate) => gate.update(pins),
			Part::Chip(chip) => chip.update(pins)
		}
	}
}



