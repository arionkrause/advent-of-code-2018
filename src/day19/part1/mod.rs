use regex::Regex;

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    input_a: usize,
    input_b: usize,
    output: usize,
}

#[derive(Debug)]
enum InstructionType {
    Addi,
    Addr,
    Eqrr,
    Gtrr,
    Muli,
    Mulr,
    Seti,
    Setr,
}

pub fn solve(input: &str) -> usize {
    let mut registers = [0; 6];
    let (instruction_pointer_index, instructions) = decode_input(&input);
    let mut instruction_pointer = 0;

    loop {
        registers[instruction_pointer_index] = instruction_pointer;

        match instructions[instruction_pointer].instruction_type {
            InstructionType::Addi => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] + instructions[instruction_pointer].input_b;
            },
            InstructionType::Addr => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] + registers[instructions[instruction_pointer].input_b];
            },
            InstructionType::Eqrr => {
                registers[instructions[instruction_pointer].output] = if registers[instructions[instruction_pointer].input_a] == registers[instructions[instruction_pointer].input_b] {
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
            InstructionType::Mulr => {
                registers[instructions[instruction_pointer].output] = registers[instructions[instruction_pointer].input_a] * registers[instructions[instruction_pointer].input_b];
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

        if instruction_pointer >= instructions.len() {
            eprintln!("registers = {:?}", registers);
            return registers[0]
        }
    }
}

fn decode_input(input: &str) -> (usize, Vec<Instruction>) {
    let re_instruction_pointer = Regex::new(r"#ip (\d)").unwrap();
    let re_instruction = Regex::new(r"(\w{4}) (\d+) (\d+) (\d+)").unwrap();

    let capture = re_instruction_pointer.captures(&input).unwrap();
    let instruction_pointer = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();

    let mut instructions = Vec::new();

    for capture in re_instruction.captures_iter(&input) {
        instructions.push(Instruction {
            instruction_type: match capture.get(1).unwrap().as_str() {
                "addi" => InstructionType::Addi,
                "addr" => InstructionType::Addr,
                "eqrr" => InstructionType::Eqrr,
                "gtrr" => InstructionType::Gtrr,
                "muli" => InstructionType::Muli,
                "mulr" => InstructionType::Mulr,
                "seti" => InstructionType::Seti,
                "setr" => InstructionType::Setr,
                _ => panic!(),
            },
            input_a: capture.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            input_b: capture.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            output: capture.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        });
    }

    (instruction_pointer, instructions)
}

#[cfg(test)]
mod test;
