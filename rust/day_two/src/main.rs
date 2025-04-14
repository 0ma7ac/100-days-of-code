// Day 3: Exploring Rust functions
fn main() {
    let arr  = [1,2,3,5];
    let tupl = ("Omar", 2, "AHmed");
    println!("Hello, world!");

    println!("array : {:?}", arr);
    println!("Tuple : {:?}", tupl);
    let result = add(4, 7);
    println!("Result is = {}", result);
    greeting("Omar", 1);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greeting(name: &str, age: i32) {
    println!("Hello {}!, you are {} years old", name, age);
}

