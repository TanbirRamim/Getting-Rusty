fn main() {
    let mut i = 0;
    loop {
        println!("{:?}", i);
        i += 1;
        if i == 10 {
            println!("matched the criteria");
            break;
        }
    }
    println!("Done!");
}
