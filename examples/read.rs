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
}
