// Bubble sort

use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // Get the number of items and max value.
    let num_items = get_i32("# Items: ");
    let max = get_i32("Max: ");

    // Make and display the unsorted vector.
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, 999);
    println!();

    // Sort and display the result.
    bubble_sort(&mut vec);
    print_vec(&vec, 999);

    // Verify that it's sorted.
    check_sorted(&vec);
}

// Make a vector of random i32 values in the range [0 and max).
fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    return vec;
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
    println!("{string}");
}

// Use bubble sort to sort the vector.
fn bubble_sort(vec: &mut Vec<i32>) {
    let mut swapped = true;
    let mut u = 1;
    let mut v = vec.len();
    while swapped {
        swapped = false;
        if u % 2 > 0 {
            for i in u..v {
                // If vec[i - 1] and vec[i] are out of order ...
                if vec[i - 1] > vec[i] {
                    // ... swap them.
                    (vec[i - 1], vec[i]) = (vec[i], vec[i - 1]);
                    swapped = true;
                }
            }
            v -= 1;
        } else {
            for i in (u..v).rev() {
                // If vec[i] and vec[i - 1] are out of order ...
                if vec[i] < vec[i - 1] {
                    // ... swap them.
                    (vec[i], vec[i - 1]) = (vec[i - 1], vec[i]);
                    swapped = true;
                }
            }
            u += 1;
        }
    }
}
// Verify that the Vec is sorted.
fn check_sorted(vec: &Vec<i32>) {
    for i in 1usize..vec.len() {
        if vec[i - 1] > vec[i] {
            println!("SORRY Sir, The vector is NOT sorted!");
            return;
        }
    }
    println!("\nThe vector is NOW sorted!");
}

// *****************
// *** Utilities ***
// *****************
// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    return trimmed.parse::<i32>().expect("Error parsing integer");
}

// ************
// *** Prng ***
// ************
struct Prng {
    seed: u32,
}

impl Prng {
    fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();
        return prng;
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    // Return a pseudorandom value in the range [0, 2147483647].
    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;
        return self.seed;
    }

    // Return a pseudorandom value in the range [0.0, 1.0).
    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        return f / (2147483647.0 + 1.0);
    }

    // Return a pseudorandom value in the range [min, max).
    fn next_i32(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();
        return result as i32;
    }
}
