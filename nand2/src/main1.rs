union Myunion {
    f1: i32,
    f2: i32,
}

fn main() {
    let u = Myunion { f1: 123 };
    unsafe {
        println!("{}", u.f1);
    }
}
