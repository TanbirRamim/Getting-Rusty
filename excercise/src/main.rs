fn main() {
    let mut x: i8 = 7;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" ->> {x}");
    }
    println!();
}
// just to add and commit, pardon please :'3
// title ends here
