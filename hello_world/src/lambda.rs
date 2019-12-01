fn func<F>(start: i64, end: i64, f: F) where F: Fn(i64) -> bool {
    for i in start..end + 1 {
        println!("{}? {:?}", i, f(i));
    }
}