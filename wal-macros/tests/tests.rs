#[test]
fn html_macro_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/html_macro/single-value-pass.rs");
    t.compile_fail("tests/html_macro/single-value-fail.rs");
    t.pass("tests/html_macro/single-node-pass.rs");
    t.pass("tests/html_macro/single-value-within-node-pass.rs");
    t.compile_fail("tests/html_macro/single-value-within-node-fail.rs");
    t.compile_fail("tests/html_macro/node-without-closing-fail.rs");
    t.compile_fail("tests/html_macro/node-with-wrong-way-of-closing-fail.rs");
    t.pass("tests/html_macro/passing-function-pass.rs");
    t.compile_fail("tests/html_macro/passing-function-fail.rs");
    t.pass("tests/html_macro/list-pass.rs");
    t.compile_fail("tests/html_macro/list-fail.rs");
}
