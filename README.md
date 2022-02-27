# charming_polygon

This is a tiny library for representing simple polygons as text. 
For example, an 8x3 rectangle can be writen as:
```
0      1

3      2
```
The only valid characters to represent vertices are ASCII alphanumeric. 
Vertices are arranged in order of their ASCII values, i.e. 0..9, A..Z, a..z.

Add charming_polygon as a dependency to Cargo.toml:
```toml
[dependencies]
early_ketupa = { git = "https://github.com/30bit/charming_polygon" }
```
Read text to an array of points:
```rust
use charming_polygon::Buffer;

fn main() {
    let buffer = Buffer::from_str(
        "
        0          1
                            2
              7       3
                                    4
                5
    6
    ",
    );
    println!("{:?}", buffer.points())
    // prints:
    // [[4, 0], [15, 0], [24, 1], [18, 2], [32, 3], [12, 4], [0, 5], [10, 2]]
}
```
Write an array of points to string:
```rust
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
    // prints:
    //    0          1     
    //                        2   
    //          7       3            
    //                                4
    //            5               
    //6                          
}
```
