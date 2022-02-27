use charming_polygon::{Buffer, Writer};

fn main() {
    let buffer = Buffer::from_points([
        [4, 0],
        [15, 0],
        [24, 1],
        [18, 2],
        [32, 3],
        [12, 4],
        [0, 5],
        [10, 2],
    ]);
    let mut string = String::new();
    Writer::new().write(&mut string, &buffer);
    string.lines().for_each(|line| println!("{line}"));
}
