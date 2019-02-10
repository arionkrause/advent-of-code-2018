use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    input_a: usize,
    input_b: usize,
    output: usize,
    intelligible: String,
}

#[derive(Debug)]
enum InstructionType {
    Addi,
    Addr,
    Bani,
    Bori,
    Eqri,
    Eqrr,
    Gtir,
    Gtrr,
    Muli,
    Seti,
    Setr,
}

pub fn solve(input: &str) -> usize {
    let (instruction_pointer_index, instructions) = decode_input(&input);
    let mut instruction_pointer = 0;
    let mut registers = [0; 6];
    let mut seem_values = HashSet::new();
    let mut last_seem_value = None;

    loop {
        registers[instruction_pointer_index] = instruction_pointer;

//        eprintln!("{:12?}, ip: {:width$}{:fill$}, instruction: {}", registers, instruction_pointer, String::from(" "), instructions[instruction_pointer].intelligible, width = instruction_pointer + 1, fill = 32 - instruction_pointer);

        if instruction_pointer == 28 {
            if seem_values.contains(&registers[1]) {
                return last_seem_value.unwrap();
            }

            seem_values.insert(registers[1]);
            last_seem_value = Some(registers[1]);
//            eprintln!("seem_values = {:?}", seem_values);
        }

        match instructions[instruction_pointer].instruction_type {
            InstructionType::Addi => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] + instructions[instruction_pointer].input_b;
            },
            InstructionType::Addr => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] + registers[instructions[instruction_pointer].input_b];
            },
            InstructionType::Bani => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] & instructions[instruction_pointer].input_b;
            },
            InstructionType::Bori => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] | instructions[instruction_pointer].input_b;
            },
            InstructionType::Eqri => {
                registers[instructions[instruction_pointer].output] = if registers[instructions[instruction_pointer].input_a] == instructions[instruction_pointer].input_b {
                    1
                } else {
                    0
                };
            },
            InstructionType::Eqrr => {
                registers[instructions[instruction_pointer].output] = if registers[instructions[instruction_pointer].input_a] == registers[instructions[instruction_pointer].input_b] {
                    1
                } else {
                    0
                };
            },
            InstructionType::Gtir => {
                registers[instructions[instruction_pointer].output] = if instructions[instruction_pointer].input_a > registers[instructions[instruction_pointer].input_b] {
                    1
                } else {
                    0
                };
            },
            InstructionType::Gtrr => {
                registers[instructions[instruction_pointer].output] = if registers[instructions[instruction_pointer].input_a] > registers[instructions[instruction_pointer].input_b] {
                    1
                } else {
                    0
                };
            },
            InstructionType::Muli => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] * instructions[instruction_pointer].input_b;
            },
            InstructionType::Seti => {
                registers[instructions[instruction_pointer].output] = instructions[instruction_pointer].input_a;
            },
            InstructionType::Setr => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a];
            },
        };

        instruction_pointer = registers[instruction_pointer_index];
        instruction_pointer += 1;
    }
}

fn decode_input(input: &str) -> (usize, Vec<Instruction>) {
    let re_instruction_pointer = Regex::new(r"#ip (\d)").unwrap();
    let re_instruction = Regex::new(r"(\w{4}) (\d+) (\d+) (\d+) +// (.+)\r\n").unwrap();

    let capture = re_instruction_pointer.captures(&input).unwrap();
    let instruction_pointer = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let mut instructions = Vec::new();

    for capture in re_instruction.captures_iter(&input) {
        instructions.push(Instruction {
            instruction_type: match capture.get(1).unwrap().as_str() {
                "addi" => InstructionType::Addi,
                "addr" => InstructionType::Addr,
                "bani" => InstructionType::Bani,
                "bori" => InstructionType::Bori,
                "eqri" => InstructionType::Eqri,
                "eqrr" => InstructionType::Eqrr,
                "gtir" => InstructionType::Gtir,
                "gtrr" => InstructionType::Gtrr,
                "muli" => InstructionType::Muli,
                "seti" => InstructionType::Seti,
                "setr" => InstructionType::Setr,
                _ => panic!(),
            },
            input_a: capture.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            input_b: capture.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            output: capture.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            intelligible: capture.get(5).unwrap().as_str().to_string(),
        });
    }

    (instruction_pointer, instructions)
}
