use charming_polygon::Buffer;

#[test]
fn from_points_test() {
    let mut buffer = Buffer::from_points([[0, 0]]);
    assert_eq!(buffer.width(), 1);
    assert_eq!(buffer.height(), 1);
    assert_eq!(buffer.points(), &[[0, 0]]);
    buffer.set_from_points([[1, 1]]);
    assert_eq!(buffer.width(), 1);
    assert_eq!(buffer.height(), 1);
    assert_eq!(buffer.points(), &[[0, 0]]);
    buffer.set_from_points([[1, 1], [3, 3]]);
    assert_eq!(buffer.width(), 3);
    assert_eq!(buffer.height(), 3);
    assert_eq!(buffer.points(), &[[0, 0], [2, 2]]);
    buffer.set_from_points([]);
    assert_eq!(buffer.width(), 0);
    assert_eq!(buffer.height(), 0);
    assert_eq!(buffer.points(), &[[0u16; 2]; 0]);
}

#[test]
fn from_str_test() {
    let mut buffer = Buffer::from_str("0");
    assert_eq!(buffer.width(), 1);
    assert_eq!(buffer.height(), 1);
    assert_eq!(buffer.points(), &[[0, 0]]);
    buffer.set_from_str("01");
    assert_eq!(buffer.width(), 2);
    assert_eq!(buffer.height(), 1);
    assert_eq!(buffer.points(), &[[0, 0], [1, 0]]);
    buffer.set_from_str("0\n1");
    assert_eq!(buffer.width(), 1);
    assert_eq!(buffer.height(), 2);
    assert_eq!(buffer.points(), &[[0, 0], [0, 1]]);
    buffer.set_from_str("0 1");
    assert_eq!(buffer.width(), 3);
    assert_eq!(buffer.height(), 1);
    assert_eq!(buffer.points(), &[[0, 0], [2, 0]]);
    buffer.set_from_str(" 0 1\n2 3");
    assert_eq!(buffer.width(), 4);
    assert_eq!(buffer.height(), 2);
    assert_eq!(buffer.points(), &[[1, 0], [3, 0], [0, 1], [2, 1]]);
}
