use charming_polygon::{Buffer, Writer};

#[test]
fn bijective_str_test() {
    let mut string = String::new();
    let mut writer = Writer::new();
    let mut buffer = Buffer::from_str("0");
    writer.write(&mut string, &buffer).unwrap();
    assert_eq!(&string, "0");
    string.clear();
    buffer.set_from_str("01");
    writer.write(&mut string, &buffer).unwrap();
    assert_eq!(&string, "01");
    string.clear();
    buffer.set_from_str("0\n1");
    writer.write(&mut string, &buffer).unwrap();
    assert_eq!(&string, "0\n1");
    string.clear();
    buffer.set_from_str("0 1");
    writer.write(&mut string, &buffer).unwrap();
    assert_eq!(&string, "0 1");
    string.clear();
    buffer.set_from_str(" 0 1\n2 3 ");
    writer.write(&mut string, &buffer).unwrap();
    assert_eq!(&string, " 0 1\n2 3 ");
}
