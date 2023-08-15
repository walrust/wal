#[test]
fn html_macro_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/html_macro/single-value-pass.rs");
    //t.compile_fail("tests/html_macro/single-value-fail.rs");
    t.pass("tests/html_macro/single-element-pass.rs");
    t.pass("tests/html_macro/single-value-within-element-pass.rs");
    //t.compile_fail("tests/html_macro/single-value-within-element-fail.rs");
    t.pass("tests/html_macro/element-tree-pass.rs");
    //t.compile_fail("tests/html_macro/element-without-closing-fail.rs");
    //t.compile_fail("tests/html_macro/element-without-opening-fail.rs");
    t.compile_fail("tests/html_macro/element-with-wrong-way-of-closing-fail.rs");
    //t.pass("tests/html_macro/passing-function-pass.rs");
    //t.compile_fail("tests/html_macro/passing-function-fail.rs");
    //t.pass("tests/html_macro/attributes-pass.rs");
    //t.pass("tests/html_macro/list-of-elements-pass.rs");
    //t.compile_fail("tests/html_macro/list-of-elements-fail.rs");
    //t.pass("tests/html_macro/for-loop-pass.rs");
    //t.compile_fail("tests/html_macro/for-loop-fail.rs");
    //t.pass("tests/html_macro/if-pass.rs");
    //t.compile_fail("tests/html_macro/if-fail.rs");
    //t.pass("tests/html_macro/if-let-pass.rs");
    //t.compile_fail("tests/html_macro/if-let-fail.rs");
    //t.pass("tests/html_macro/if-and-if-let-pass.rs");
}
