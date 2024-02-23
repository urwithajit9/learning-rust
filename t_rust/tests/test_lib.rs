use t_rust::add;
use t_rust::divide;
use t_rust::multiply;
use t_rust::subtract;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3)
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(1, 2), -1)
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(1, 2), 2)
}

#[test]
fn test_divide() {
    assert_eq!(divide(4, 2), 2)
}
