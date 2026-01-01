fn main() {
    let mut s1 = String::from("hello");
    {
        let r1 = s.clone();
    println!("{r1}");
    }
    let r2 = &mut s1;
    println!("{r2}");
}
