
type RGB = (i16, i16, i16);

fn color(c: &str) -> RGB {
    (1, 1, 1)
}

fn show(c: fn(&str) -> RGB) {
    println!("{:?}", std::mem::size_of_val(&c));  // 8
    println!("{:?}", c("black"));
}

fn run() {
    let rgb = color;
    show(rgb);
    println!("{:?}", std::mem::size_of_val(&rgb));  // 0
    let c : fn(&str) -> RGB = rgb;
}
