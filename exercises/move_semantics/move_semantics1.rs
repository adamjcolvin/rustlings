// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    // This is a simple example of moving ownership. vec0 is declared here and is immutable.
    let vec0 = vec![22, 44, 66];

    //vec0 is then moved to the fill_vec function, meaning that vec0 is no longer valid here.
    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    //vec is mutable here, so we can push to it.
    let mut vec = vec;

    vec.push(88);

    vec
}
