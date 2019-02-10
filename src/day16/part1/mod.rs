use regex::Regex;

#[derive(Debug)]
struct Sample {
    registers_before: Vec<u8>,
    instruction: Instruction,
    registers_after: Vec<u8>,
}

#[derive(Debug)]
struct Instruction {
    opcode: u8,
    input_a: u8,
    input_b: u8,
    output: u8,
}

pub fn solve(input: &str) -> usize {
    let samples = decode_input(&input);
    let mut samples_behaving_like_three_opcodes = 0;

    for sample in samples.iter() {
        let mut results = Vec::new();
        results.push(addr(&sample));
        results.push(addi(&sample));
        results.push(mulr(&sample));
        results.push(muli(&sample));
        results.push(banr(&sample));
        results.push(bani(&sample));
        results.push(borr(&sample));
        results.push(bori(&sample));
        results.push(setr(&sample));
        results.push(seti(&sample));
        results.push(gtir(&sample));
        results.push(gtri(&sample));
        results.push(gtrr(&sample));
        results.push(eqir(&sample));
        results.push(eqri(&sample));
        results.push(eqrr(&sample));

        if results.iter().filter(|&r| r == &sample.registers_after).count() >= 3 {
            samples_behaving_like_three_opcodes += 1;
        }
    }

    samples_behaving_like_three_opcodes
}

fn decode_input(input: &str) -> Vec<Sample> {
    let mut samples = Vec::new();
    let re = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)]\r?\n(\d+) (\d) (\d) (\d)\r?\nAfter: {2}\[(\d), (\d), (\d), (\d)]").unwrap();

    for capture in re.captures_iter(&input) {
        samples.push(Sample {
            registers_before: vec![capture.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                                   capture.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                                   capture.get(3).unwrap().as_str().parse::<u8>().unwrap(),
                                   capture.get(4).unwrap().as_str().parse::<u8>().unwrap(),
            ],
            instruction: Instruction {
                opcode: capture.get(5).unwrap().as_str().parse::<u8>().unwrap(),
                input_a: capture.get(6).unwrap().as_str().parse::<u8>().unwrap(),
                input_b: capture.get(7).unwrap().as_str().parse::<u8>().unwrap(),
                output: capture.get(8).unwrap().as_str().parse::<u8>().unwrap()
            },
            registers_after: vec![capture.get(9).unwrap().as_str().parse::<u8>().unwrap(),
                                  capture.get(10).unwrap().as_str().parse::<u8>().unwrap(),
                                  capture.get(11).unwrap().as_str().parse::<u8>().unwrap(),
                                  capture.get(12).unwrap().as_str().parse::<u8>().unwrap(),
            ],
        });
    }

    samples
}

fn addr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] + registers_after[sample.instruction.input_b as usize];
    registers_after
}

fn addi(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] + sample.instruction.input_b;
    registers_after
}

fn mulr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] * registers_after[sample.instruction.input_b as usize];
    registers_after
}

fn muli(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] * sample.instruction.input_b;
    registers_after
}

fn banr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] & registers_after[sample.instruction.input_b as usize];
    registers_after
}

fn bani(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] & sample.instruction.input_b;
    registers_after
}

fn borr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] | registers_after[sample.instruction.input_b as usize];
    registers_after
}

fn bori(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize] | sample.instruction.input_b;
    registers_after
}

fn setr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = registers_after[sample.instruction.input_a as usize];
    registers_after
}

fn seti(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();
    registers_after[sample.instruction.output as usize] = sample.instruction.input_a;
    registers_after
}

fn gtir(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();

    registers_after[sample.instruction.output as usize] = if sample.instruction.input_a > registers_after[sample.instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn gtri(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();

    registers_after[sample.instruction.output as usize] = if registers_after[sample.instruction.input_a as usize] > sample.instruction.input_b {
         1
    } else {
        0
    };

    registers_after
}

fn gtrr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();

    registers_after[sample.instruction.output as usize] = if registers_after[sample.instruction.input_a as usize] > registers_after[sample.instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn eqir(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();

    registers_after[sample.instruction.output as usize] = if sample.instruction.input_a == registers_after[sample.instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn eqri(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();

    registers_after[sample.instruction.output as usize] = if registers_after[sample.instruction.input_a as usize] == sample.instruction.input_b {
        1
    } else {
        0
    };

    registers_after
}

fn eqrr(sample: &Sample) -> Vec<u8> {
    let mut registers_after = sample.registers_before.clone();

    registers_after[sample.instruction.output as usize] = if registers_after[sample.instruction.input_a as usize] == registers_after[sample.instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

#[cfg(test)]
mod test;
