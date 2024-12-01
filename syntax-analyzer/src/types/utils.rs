pub fn is_number(input: &str) -> bool {
    input.parse::<f64>().is_ok()
}
