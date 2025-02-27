use crate::assert_erl;

#[test]
fn one_var() {
    // One var
    assert_erl!(
        r#"pub fn go() {
  let assert Ok(y) = Ok(1)
  y
}"#
    );
}

#[test]
fn more_than_one_var() {
    // More vars
    assert_erl!(
        r#"pub fn go(x) {
  let assert [1, a, b, c] = x
  [a, b, c]
}"#
    );
}

#[test]
fn pattern_let() {
    // Pattern::Let
    assert_erl!(
        r#"pub fn go(x) {
  let assert [1 as a, b, c] = x
  [a, b, c]
}"#
    );
}

#[test]
fn variable_rewrites() {
    // Following asserts use appropriate variable rewrites
    assert_erl!(
        r#"pub fn go() {
  let assert Ok(y) = Ok(1)
  let assert Ok(y) = Ok(1)
  y
}"#
    );
}

#[test]
fn message() {
    assert_erl!(
        r#"
pub fn unwrap_or_panic(value) {
  let assert Ok(inner) = value as "Oops, there was an error"
  inner
}
"#
    );
}

#[test]
fn variable_message() {
    assert_erl!(
        r#"
pub fn expect(value, message) {
  let assert Ok(inner) = value as message
  inner
}
"#
    );
}

// TODO: patterns that are just vars don't render a case expression
// #[test]
// fn just_pattern() {
//     assert_erl!(
//         r#"pub fn go() {
//   let assert x = Ok(1)
//   x
// }"#
//     );
// }
