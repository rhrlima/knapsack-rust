use rstest::rstest;

use knapsack::problem::{read_instance, Instance};

#[test]
fn read_valid_file() {
    assert_eq!(
        Instance {
            num_items: 2,
            max_weight: 99.0,
            profits: vec![1.0, 3.0],
            weights: vec![2.0, 4.0],
            optimal: 10.0
        },
        read_instance("tests/assets/valid").unwrap()
    );
}

#[test]
fn read_nonexisting_file() {
    let e = read_instance("tests/assets/nofile").unwrap_err();
    assert_eq!(std::io::ErrorKind::NotFound, e.kind());
}

#[rstest]
#[case("tests/assets/empty")]
#[case("tests/assets/first_line_empty")]
#[case("tests/assets/second_line_empty")]
#[case("tests/assets/first_line_invalid_1")]
#[case("tests/assets/first_line_invalid_2")]
#[case("tests/assets/second_line_invalid_1")]
#[case("tests/assets/second_line_invalid_2")]
fn read_invalid_file(#[case] filename: &str) {
    println!("{}", filename);
    let e = read_instance(filename).unwrap_err();
    assert_eq!(std::io::ErrorKind::InvalidData, e.kind());
}
