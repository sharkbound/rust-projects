pub fn number_sign(val: i32) -> &'static str {
    if val >= 0 { "+" } else { "-" }
}