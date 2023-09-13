fn main() {
    // snake case: all lower case, use underscore _
    println!("Hello, world!");

    another_function_0();
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = 6;          // is a statement
    // Statements do not return values. Therefore, you can’t assign a let statement to another variable
    // let x = (let y = 9);
    // Note: In C, we can use x = y = 6; That's because the assignment returns the value of the assignment

    // A new scope block created with curly brackets is an expression
    // Its last line determines its value
    let y = {
        let x = 3;
        // If you add a semicolon to the end of an expression, 
        // you turn it into a statement, and it will then not return a value.
        x + 1
    };    // y = 4

    println!("The value of y is {y}");

    let x = five();
    let y = plus_one(x);
    println!("The value of x is {x}, y is {y}");


}

// 1. Arguments
fn another_function_0() {
    println!("Another function.");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


/**
 * 2. Statements and Expressions
 * (1) Statements: instructions that perform some action and do not return a value
 * (2) Expressions: evaluate to a resultant value
 * Rm: In Scala, all statements are expressions that return a value. 
 *      The value of the last statement determines the value of an expression.
 *      The if expression is no different — it always returns a value
 * Examples:
 * Statements
 * (1) Creating a variable and assigning a value to it with the let keyword is a statement
 * (2) Function definitions are also statements; the entire preceding example is a statement in itself
 * Expressions
 * (3) math operations, like 5 + 6 are expressions
 * (4) Calling a function is an expression. 
 * (5) Calling a macro is an expression. 
 * (6) A new scope block created with curly brackets is an expression
 */


// 3. Functions with Return Values (-> type)
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
