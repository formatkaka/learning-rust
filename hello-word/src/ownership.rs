fn main() {
    let s1 = String::from("hello world");
    println!("String length is {}", calc_length(s1));
}

fn calc_length(s: &String) -> usize {
    s.len();
}
