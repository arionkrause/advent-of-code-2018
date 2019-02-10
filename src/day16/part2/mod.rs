use regex::Regex;
use std::collections::HashMap;

static AMOUNT_OPCODES: usize = 16;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Sample {
    registers_before: Vec<u32>,
    instruction: Instruction,
    registers_after: Vec<u32>,
    candidate_operations: Vec<Operation>,
}

impl Sample {
    fn brute_force_combinations(&mut self, remaining_samples: Vec<Sample>, mut combinations: HashMap<u32, Operation>) -> Result<HashMap<u32, Operation>, &'static str> {
        if combinations.get(&self.instruction.opcode).is_some() {
            if self.candidate_operations.contains(combinations.get(&self.instruction.opcode).unwrap()) {
                let mut remaining_samples_copy = remaining_samples.to_vec();

                match remaining_samples_copy.pop() {
                    Some(mut remaining_sample) => {
                        let remaining_sample_deduction = remaining_sample.brute_force_combinations(remaining_samples_copy, combinations.clone());

                        if remaining_sample_deduction.is_ok() {
                            combinations.extend(remaining_sample_deduction.unwrap());
                            return Ok(combinations);
                        };
                    },
                    _ => {
                        return Ok(combinations);
                    },
                };
            }

            return Err("Invalid combination.");
        }

        for candidate_operation in self.candidate_operations.iter() {
            if combinations.iter().find(|(_, operation)| operation == &candidate_operation).is_some() {
                continue;
            };

            let mut remaining_samples_copy = remaining_samples.to_vec();
            let mut combinations_try = combinations.clone();
            combinations_try.insert(self.instruction.opcode, candidate_operation.clone());

            match remaining_samples_copy.pop() {
                Some(mut remaining_sample) => {
                    let remaining_sample_deduction = remaining_sample.brute_force_combinations(remaining_samples_copy, combinations_try.clone());

                    if remaining_sample_deduction.is_ok() {
                        combinations_try.extend(remaining_sample_deduction.unwrap());
                        return Ok(combinations_try);
                    };
                },
                _ => return Ok(combinations_try),
            };
        };

        return Err("No valid combination found.");
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Instruction {
    opcode: u32,
    input_a: u32,
    input_b: u32,
    output: u32,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Operation {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

pub fn solve(input: &str) -> usize {
    let mut samples = decode_samples(&input);

    for sample in samples.iter_mut() {
        let mut results = Vec::new();
        results.push((Operation::Addr, addr(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Addi, addi(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Mulr, mulr(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Muli, muli(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Banr, banr(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Bani, bani(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Borr, borr(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Bori, bori(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Setr, setr(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Seti, seti(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Gtir, gtir(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Gtri, gtri(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Gtrr, gtrr(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Eqir, eqir(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Eqri, eqri(&sample.registers_before, &sample.instruction)));
        results.push((Operation::Eqrr, eqrr(&sample.registers_before, &sample.instruction)));

        sample.candidate_operations.extend(results.iter()
                .filter(|(_, result)| result == &sample.registers_after)
                .map(|(operation, _)| operation)
                .cloned()
                .collect::<Vec<Operation>>()
        );
    }

    let combinations = if there_are_samples_with_only_one_operation_candidate(&samples) {
        get_combinations_by_exclusion(&samples)
    } else {
        samples.sort_by(|a, b| b.candidate_operations.len().cmp(&a.candidate_operations.len()));
        samples.pop().unwrap().brute_force_combinations(samples.to_vec(), HashMap::new()).unwrap_or_default()
    };

    let test_program = decode_test_program(&input);
    run_program(&test_program, &combinations)
}

fn decode_samples(input: &str) -> Vec<Sample> {
    let mut samples = Vec::new();
    let re = Regex::new(r"Before: \[(\d), (\d), (\d), (\d)]\r?\n(\d+) (\d) (\d) (\d)\r?\nAfter: {2}\[(\d), (\d), (\d), (\d)]").unwrap();

    for capture in re.captures_iter(&input) {
        samples.push(Sample {
            registers_before: vec![capture.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                                   capture.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                                   capture.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                                   capture.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            ],
            instruction: Instruction {
                opcode: capture.get(5).unwrap().as_str().parse::<u32>().unwrap(),
                input_a: capture.get(6).unwrap().as_str().parse::<u32>().unwrap(),
                input_b: capture.get(7).unwrap().as_str().parse::<u32>().unwrap(),
                output: capture.get(8).unwrap().as_str().parse::<u32>().unwrap()
            },
            registers_after: vec![capture.get(9).unwrap().as_str().parse::<u32>().unwrap(),
                                  capture.get(10).unwrap().as_str().parse::<u32>().unwrap(),
                                  capture.get(11).unwrap().as_str().parse::<u32>().unwrap(),
                                  capture.get(12).unwrap().as_str().parse::<u32>().unwrap(),
            ],
            candidate_operations: Vec::new(),
        });
    }

    samples
}

fn there_are_samples_with_only_one_operation_candidate(samples: &Vec<Sample>) -> bool {
    samples.iter()
            .find(|s| s.candidate_operations.len() == 1)
            .is_some()
}

fn get_combinations_by_exclusion(samples: &Vec<Sample>) -> HashMap<u32, Operation> {
    let mut combinations = HashMap::new();

    'outer: loop {
        for sample in samples.iter() {
            let available_candidates = sample.candidate_operations.iter()
                    .filter(|candidate_operation| combinations.iter()
                            .find(|(_, operation)| operation == candidate_operation).is_none())
                    .cloned()
                    .collect::<Vec<Operation>>();

            if available_candidates.len() == 1 {
                combinations.insert(sample.instruction.opcode, available_candidates[0].clone());

                if combinations.len() == AMOUNT_OPCODES {
                    break 'outer;
                }
            }
        }
    }

    combinations
}

fn decode_test_program(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    let re = Regex::new(r"(\d+) (\d) (\d) (\d)\r\n").unwrap();

    for capture in re.captures_iter(&input[input.rfind("]").unwrap() + 1..]) {
        instructions.push(Instruction {
                opcode: capture.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                input_a: capture.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                input_b: capture.get(3).unwrap().as_str().parse::<u32>().unwrap(),
                output: capture.get(4).unwrap().as_str().parse::<u32>().unwrap()
        });
    }

    instructions
}

fn addr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] + registers_after[instruction.input_b as usize];
    registers_after
}

fn addi(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] + instruction.input_b;
    registers_after
}

fn mulr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] * registers_after[instruction.input_b as usize];
    registers_after
}

fn muli(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] * instruction.input_b;
    registers_after
}

fn banr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] & registers_after[instruction.input_b as usize];
    registers_after
}

fn bani(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] & instruction.input_b;
    registers_after
}

fn borr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] | registers_after[instruction.input_b as usize];
    registers_after
}

fn bori(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize] | instruction.input_b;
    registers_after
}

fn setr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = registers_after[instruction.input_a as usize];
    registers_after
}

fn seti(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();
    registers_after[instruction.output as usize] = instruction.input_a;
    registers_after
}

fn gtir(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();

    registers_after[instruction.output as usize] = if instruction.input_a > registers_after[instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn gtri(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();

    registers_after[instruction.output as usize] = if registers_after[instruction.input_a as usize] > instruction.input_b {
        1
    } else {
        0
    };

    registers_after
}

fn gtrr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();

    registers_after[instruction.output as usize] = if registers_after[instruction.input_a as usize] > registers_after[instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn eqir(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();

    registers_after[instruction.output as usize] = if instruction.input_a == registers_after[instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn eqri(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();

    registers_after[instruction.output as usize] = if registers_after[instruction.input_a as usize] == instruction.input_b {
        1
    } else {
        0
    };

    registers_after
}

fn eqrr(registers_before: &Vec<u32>, instruction: &Instruction) -> Vec<u32> {
    let mut registers_after = registers_before.clone();

    registers_after[instruction.output as usize] = if registers_after[instruction.input_a as usize] == registers_after[instruction.input_b as usize] {
        1
    } else {
        0
    };

    registers_after
}

fn run_program(program: &Vec<Instruction>, combinations: &HashMap<u32, Operation>) -> usize {
    let mut registers = vec![0, 0, 0, 0];

    for instruction in program {
        registers = match combinations.get(&instruction.opcode).unwrap() {
            Operation::Addr => addr(&registers, &instruction),
            Operation::Addi => addi(&registers, &instruction),
            Operation::Mulr => mulr(&registers, &instruction),
            Operation::Muli => muli(&registers, &instruction),
            Operation::Banr => banr(&registers, &instruction),
            Operation::Bani => bani(&registers, &instruction),
            Operation::Borr => borr(&registers, &instruction),
            Operation::Bori => bori(&registers, &instruction),
            Operation::Setr => setr(&registers, &instruction),
            Operation::Seti => seti(&registers, &instruction),
            Operation::Gtir => gtir(&registers, &instruction),
            Operation::Gtri => gtri(&registers, &instruction),
            Operation::Gtrr => gtrr(&registers, &instruction),
            Operation::Eqir => eqir(&registers, &instruction),
            Operation::Eqri => eqri(&registers, &instruction),
            Operation::Eqrr => eqrr(&registers, &instruction),
        }
    }

    registers[0] as usize
}
