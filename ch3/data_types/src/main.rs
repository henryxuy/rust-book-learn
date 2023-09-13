/**
 * Part A Scalar
 * 4 scalar types: integers, floating-point numbers, Booleans, and characters
 * 
 * 1. Integer types: signed, unsigned
 * signed: i8, i16, i32, i64, i128, isize (arch) - two's complement
 * unsigned: u8, u16, u32, u64, u128, usize (arch)
 * Number literals: allow type suffix, e.g. 57u8 => unsigned 8 bit integer
 *  (1) Decimal: 98_222
 *  (2) Hex: 0xff
 *  (3) Octal: 0o77
 *  (4) Binary: 0b1111_0000
 *  (5) Byte (u8 only): b'A'
 *  number: default is i32, index for set: default is isize or usize
 * 
 * In debug mode, overflow checking is applied and int overflow would lead to panic
 * In --release mode, would use two's complement wrapping instead of panic
 *      e.g. 256 -> 0, 257 -> 1
 * 
 * Related methods to control this behavior:
 *      (1) wrapping_*: method using two's complement wrapping
 *      (2) checked_*: return None if overflow
 *      (3) overflowing_*: return value, bool to indicate whether overflow or not
 *      (4) saturating_*: Saturate at the value’s minimum or maximum values
 * 
 * 2. Floating-Point Types: f32, f64
 * 
 * 3. The Boolean Type: bool -> true, false
 * 
 * 4. The Character Type: char
 */

/**
 * Part B: Compound types:
 * 1. tuple
 * 
 * 2. array
 * 
 */

use std::io;

const TWO: u32 = 1 + 1;

fn main() {
    // println!("Hello, world!");

    // Part A: Scalar
    let x = 2.0;            // by default it's f64
    let y: f32 = 3.0;

    // Numeric operations: +, -, *, /, %
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    let mut x = 1;
    println!("{x}");
    x += 1;
    println!("{x}");

    println!("{TWO}");

    println!("{x}, {y}, {sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");
    println!("{t}, {f}, {c}, {z}, {heart_eyed_cat}");

    // Part B: Compound types
    // 1. tup
    // can use pattern matching to destruct a tup
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // can also use dot (.idx) to access tuple members
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // tuple without any values has a special name, unit, ()
    // Expressions implicitly return the unit value if they don’t return any other value.
    let a_unit = ();

    // 2. array [type; len]: length is fixed. 
    // Use it if you want to (1) let stack store your data. (2) keep fixed number of elements
    // in std, `vector` => variable length

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // initialize array with the same elements: let arr = [val; number of elements]
    let a = [3; 5];             // same as: let a = [3, 3, 3, 3, 3];


    // Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // If we use an invalid index, a RUNTIME error will occur
    let element = a[index];

    println!("The value of the element at index {index} is: {element}")


}
