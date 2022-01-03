mod args;
mod name;

struct S(u32, u8);

fn main() {
    println!("Hello, world!");
    let f: bool = args::YEAR == "-Y";
    println!("{}", f);
}
