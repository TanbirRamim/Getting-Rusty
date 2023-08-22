enum Direction {
    Left,
    Right,
}

fn main() {
    let go = Direction::Right;
    match go {
        Direction::Left => println!("Go Left"),
        Direction::Right => println!("Go Right"),
    }
}
