use my_proc_macro::generate_include_str;

fn main() {
    generate_include_str!("../Cargo.toml");
    generate_include_str!("../../target/test.txt"); // not expanded in rust-analyzer
}
