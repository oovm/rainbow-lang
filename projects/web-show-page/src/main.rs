use rainbow_macro::code_to_rml;

fn main() {
    code_to_rml!("fn answer() -> u32 { 42 }");
}
