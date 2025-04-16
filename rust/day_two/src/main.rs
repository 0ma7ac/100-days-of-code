// Day 3: Exploring Rust functions
fn main() {
    #[derive(Debug)]
    struct Person { name : String, age :i32}
    let arr  = [1,2,3,5,7,8,8,9,0,09,9,7];
    let tupl = ("Omar", 2, "AHmed");
    println!("Hello, world!");

    analyze_slice(&arr);

    let myname = reverse(("Omar","Ahmed"));
    println!("{}", myname);

    println!("array : {:?}", arr);
    println!("Tuple : {:?}", tupl);
    let result = add(4, 7);
    println!("Result is = {}", result);
    greeting("Omar", 1);

    println!("{:?}", reverse(("Omar","Ahmed")) );
    let name = "Omar".to_string();
    let age = 4;
    let omar = Person{name, age};
    println!("{:?}", omar);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greeting(name: &str, age: i32) {
    println!("Hello {}!, you are {} years old", name, age);
}
fn reverse(word1: (&str, &str)) -> String {
   let (word2, word3) = word1;
    (word2.to_string() + " " + word3).to_string()
}
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}




