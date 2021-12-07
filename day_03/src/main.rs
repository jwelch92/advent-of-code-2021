use std::fs;

type BinFrequencyArray = [[u16; 2]; 12];

enum GenerationType {
    Oxygen,
    CO2,
}

fn main() {
    let contents: String = fs::read_to_string("input.txt").expect("Error opening file");
    println!("{}", contents);
    let freq = build_frequency_array(&contents);
    println!("{:?}", freq);
    let (gamma, epsilon) = extract_gamma_epsilon_from_frequency(freq);
    println!("Part 1 {}", gamma * epsilon);

    let lines: Vec<&str> = contents.lines().collect();
    let oxy = gas_generation(&lines, 0, GenerationType::Oxygen);
    let oxy_dec = from_binary(&oxy);
    let co2 = gas_generation(&lines, 0, GenerationType::CO2);
    let co2_dec = from_binary(&co2);
    // TOO LOW! 2655855
    // TOO HIGH 4835547
    // MAYBE? 4125600
    println!("Part 2 \n oxygen binary {} dec {} co2 binary {} dec {} \n Total: {}", oxy, oxy_dec, co2, co2_dec, co2_dec * oxy_dec);

}

fn gas_generation(stack: &Vec<&str>, bit_idx: usize, gen_type: GenerationType) -> String {
    if bit_idx > 11 {
        panic!("Cannot go past 12!")
    }

    if stack.len() == 1 {
        return String::from(stack[0]);
    }

    let mut new_stack: Vec<&str> = vec![];
    let mut zero_count: u32 = 0;
    let mut one_count: u32 = 0;

    for row in stack {
        if row.chars().nth(bit_idx).unwrap() == '1' {
            one_count += 1;
        } else {
            zero_count += 1;
        }
    }

    match gen_type {
        GenerationType::Oxygen => {
            if one_count >= zero_count {
                for row in stack {
                    if row.chars().nth(bit_idx).unwrap() == '1' {
                        new_stack.push(row);
                    }
                }
            } else {
                for row in stack {
                    if row.chars().nth(bit_idx).unwrap() == '0' {
                        new_stack.push(row);
                    }
                }
            }
        },
        GenerationType::CO2 =>  {
            if zero_count <= one_count {
                for row in stack {
                    if row.chars().nth(bit_idx).unwrap() == '0' {
                        new_stack.push(row);
                    }
                }
            } else {
                for row in stack {
                    if row.chars().nth(bit_idx).unwrap() == '1' {
                        new_stack.push(row);
                    }
                }
            }
        },
    }

    return gas_generation(&new_stack, bit_idx + 1, gen_type)
}


fn build_frequency_array(contents: &String) -> BinFrequencyArray {
    let mut freq: BinFrequencyArray = [[0 as u16; 2]; 12];
    for bin in contents.lines() {
        for (pos, bit) in bin.chars().enumerate() {
            match bit {
                '0' => freq[pos][0] += 1,
                '1' => freq[pos][1] += 1,
                _ => panic!("Invalid binary number! wtf!")
            }
        }
    }
    freq
}

fn extract_gamma_epsilon_from_frequency(freq: BinFrequencyArray) -> (u32, u32) {
    let mut gamma: String = String::with_capacity(12);
    let mut epsilon: String = String::with_capacity(12);
    for pos in freq.iter() {
        if pos[0] > pos[1] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    let gamma_num = from_binary(&gamma);
    let epsilon_num = from_binary(&epsilon);
    (gamma_num, epsilon_num)
}

fn from_binary(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}