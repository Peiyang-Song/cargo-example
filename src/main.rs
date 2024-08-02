fn silly_function() -> i32 {
    24
}

fn calc() -> i32 {
    let x = 1 + 2;
    x + silly_function()
}

fn main() {
    println!("{}", calc());
}
