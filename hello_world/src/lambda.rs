pub fn func<F>(start: i64, end: i64, f: F) where F: Fn(i64) -> bool {
    for i in start..end + 1 {
        println!("{}? {:?}", i, f(i));
    }
}

//pub fn apply_mut<T, F>(it: T, f: F) where F: FnMut(T) {
//    f(it);
//    it
//}