fn main() {
    let x = 5; // rustc warning
    5; // #[warn(clippy::no_effect)]
    println!("I shave yaks!");
}
