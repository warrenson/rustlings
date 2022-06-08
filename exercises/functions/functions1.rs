// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

fn main() {
    let x = call_me();
    println!("Got: {}", x);
}

fn call_me() -> String {
    const r: &str = "called";
    println!("{}", r);
    return r.to_string();
}
