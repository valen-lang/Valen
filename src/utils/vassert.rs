
pub fn vassert_one<T>(thing: impl IntoIterator<Item = T>) -> T {
    let mut iter = thing.into_iter();
    match iter.next() {
        None => panic!("Expected one element, but was empty."),
        Some(x) => {
            let extra = iter.count();
            if extra == 0 {
                x
            } else {
                panic!("Expected one element, but was size {}.", extra + 1)
            }
        }
    }
}


