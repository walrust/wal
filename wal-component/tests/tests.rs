#[test]
fn component_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/component_trait_tests/default_component_pass.rs");
}
