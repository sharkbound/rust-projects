//#[macro_export]
//macro_rules! mac_test {
//    ($x:expr) => (println!("x = {}", $x));
//    ($y:expr) => (println!("y = {}", $y));
//}

#[macro_export]
macro_rules! to_s {
    ($s:expr) => (format!("{}", $s))
}