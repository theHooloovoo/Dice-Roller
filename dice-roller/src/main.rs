extern crate rand;
use rand::{thread_rng,Rng};

use std::env;
use std::process;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
// Each die_type contains a value.
//   For die_type::constant this value is equal to its total value
//   For all the rest, this value represents the amount of dice
//   to be rolled
enum die_type {constant(u32),
               d2(u32),
               d3(u32),
               d4(u32),
               d5(u32),
               d6(u32),
               d8(u32),
               d10(u32),
               d12(u32),
               d20(u32),
               d100(u32),
}

// Add Error catching!!
fn parse_die_string(die_string: &Vec<&str>, die_list: &mut Vec<die_type>) {
    let mut die_size = 0;
    let mut amount = 0;

    for s in die_string {
        // Check if &str is die
        if s.contains("d") {
            let (left, right) = s.split_at(s.find('d').unwrap());
            // check if no dice amount is specified
            if left.is_empty() == true {
                amount = 1;
            } else {
                amount = left.parse::<u32>().unwrap();
            }
            die_size = right.trim_left_matches('d').parse::<u32>().unwrap();
            match die_size {
                2   => die_list.push(die_type::d2(amount)),
                3   => die_list.push(die_type::d3(amount)),
                4   => die_list.push(die_type::d4(amount)),
                5   => die_list.push(die_type::d5(amount)),
                6   => die_list.push(die_type::d6(amount)),
                8   => die_list.push(die_type::d8(amount)),
                10  => die_list.push(die_type::d10(amount)),
                12  => die_list.push(die_type::d12(amount)),
                20  => die_list.push(die_type::d20(amount)),
                100 => die_list.push(die_type::d100(amount)),
                _   => { println!("Warning: {} is not a valid die size", die_size);
                         println!("Valid die sizes are: d2, d3, d4, d5, d6, d8, d10, d12, d20, d100");
                         process::exit(0x0100);
                       },
            }
            
        // Check if &str is non-valid (TODO)
        // Check if &str is constant
        } else {
            amount = s.parse::<u32>().unwrap();
            die_list.push(die_type::constant(amount));
            
        }
    }
}

// Roll the given die equal to its internal value
fn roll_die(d: &die_type) -> u32 {
    let mut rng = thread_rng();
    let mut i = 0u32;
    match d {
        &die_type::constant(n) => n, // Constants aren't rerolled
        &die_type::d2(n)       => { for _ in 0..n {i += rng.gen_range(1u32, 3u32)}   i },
        &die_type::d3(n)       => { for _ in 0..n {i += rng.gen_range(1u32, 4u32)}   i },
        &die_type::d4(n)       => { for _ in 0..n {i += rng.gen_range(1u32, 5u32)}   i },
        &die_type::d5(n)       => { for _ in 0..n {i += rng.gen_range(1u32, 6u32)}   i },
        &die_type::d6(n)       => { for _ in 0..n {i += rng.gen_range(1u32, 7u32)}   i },
        &die_type::d8(n)       => { for _ in 0..n {i += rng.gen_range(1u32, 9u32)}   i },
        &die_type::d10(n)      => { for _ in 0..n {i += rng.gen_range(1u32, 11u32)}  i },
        &die_type::d12(n)      => { for _ in 0..n {i += rng.gen_range(1u32, 13u32)}  i },
        &die_type::d20(n)      => { for _ in 0..n {i += rng.gen_range(1u32, 21u32)}  i },
        &die_type::d100(n)     => { for _ in 0..n {i += rng.gen_range(1u32, 101u32)} i },
    }
}

fn gen_total(die_list: &Vec<die_type>) -> u32 {
    let mut total = 0u32;
    for n in die_list {
        total += roll_die(n);
    }
    total
}


fn main() {
    // Establish a vector of possible error strings
    let mut err_vec: Vec<String> = Vec::new();
    let mut n = 1u32;

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("This program requires an input string!");
        println!("  e.g. '4d6+3d12+30'");
        process::exit(1);
    }
    if args.len() == 3 {
        n = args[2].parse::<u32>().unwrap();
    }

    let input = args[1].to_lowercase();
    let string_list: Vec<&str> = input.split('+').collect();
    let mut dice: Vec<die_type> = Vec::with_capacity(string_list.len());

    parse_die_string(&string_list, &mut dice);

    while n > 0 {
        println!("{}", gen_total(&dice));
        n -= 1;
    }


}

