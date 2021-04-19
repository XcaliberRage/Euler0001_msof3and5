use std::time::Instant;

const MAXIMAL: u32 = 1000;
const MAXIMAL_U: usize = MAXIMAL as usize;
const FIVE: u32 = 5;
const THREE: u32 = 3;

fn complex_method() {

    let now = Instant::now();

    let mut threes = THREE;
    let mut fives = FIVE;
    let mut ms_of_five: [u32; MAXIMAL_U] = [0; MAXIMAL_U];
    let mut ms_index = 0usize;
    let mut final_val = 0u32;

    while threes < MAXIMAL {

        final_val += threes;

        // We don't want to add the multiples of 5 again
        if threes % FIVE == 0 {
            ms_of_five[ms_index] = threes;
            ms_index += 1;
        }

        threes += THREE;

    }

    while fives < MAXIMAL {

        if !ms_of_five.contains(&fives) {
            final_val += fives;
        }

        fives += FIVE;
    }

    println!("The sum of all multiples of {0} and {1} below {2} is {3}.", THREE, FIVE, MAXIMAL, final_val);
    println!("This solution was discovered in {}ns using the complex method.", now.elapsed().as_nanos());

}

fn simple_method() {

    let now = Instant::now();
    let mut total = 0u32;

    for i in 1..MAXIMAL {

        if (i % THREE != 0) & (i % FIVE != 0) {
            continue;
        }

        total += i;
    }

    println!("The sum of all multiples of {0} and {1} below {2} is {3}.", THREE, FIVE, MAXIMAL, total);
    println!("This solution was discovered in {}ns using the simple method.", now.elapsed().as_nanos());

}

fn main() {

    simple_method();

    complex_method();

}