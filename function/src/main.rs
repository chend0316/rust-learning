mod color;

// fn sum(a: i32, b: i32) -> i32 {
//     a + b
// }

#[derive(Debug)]
struct ComplexNumber(f32, f32);

impl ComplexNumber {
    fn sum(a: ComplexNumber, b: ComplexNumber) -> ComplexNumber {
        ComplexNumber(a.0 + b.0, a.1 + b.1)
    }

    fn add(& mut self, other: ComplexNumber) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

fn main() {
    let a = ComplexNumber(1.0, 2.0);
    let b = ComplexNumber(3.0, 4.0);
    let mut c = ComplexNumber::sum(a, b);
    c.add(ComplexNumber(1.0, 1.0));
    println!("{:?}", c);

    let sum = ComplexNumber::sum;
    println!("{:?}", sum(ComplexNumber(1.0, 2.0), ComplexNumber(3.0, 4.0)));

    let add = ComplexNumber::add;
    let mut a = ComplexNumber(1.0, 2.0);
    add(& mut a, ComplexNumber(3.0, 4.0));
    println!("{:?}", a);
}
