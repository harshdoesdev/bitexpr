fn main() {
    let expression = r#"(2 + 3 * sin(π/4)) / (sqrt(9) + log(100, 10)) - 2^3"#;
    let tokens = bitexpr::tokenizer::tokenize(expression).unwrap();

    println!("{:?}", tokens);
}
