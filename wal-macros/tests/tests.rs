#[test]
fn html_macro_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/html_macro/single-value-pass.rs");
    t.pass("tests/html_macro/single-value-within-element-pass.rs");
}
