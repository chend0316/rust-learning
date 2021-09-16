fn counter1(i: i32) -> fn(i32) => i32 {
    fn inc(n: i32) -> i32 {
        n + i
    }
    inc
}

fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    move |n| n + i
}

fn main() {
    let f=counter(2);
    assert_eq!(3, f(1));
}