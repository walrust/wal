#[test]
fn html_macro_tests() {
    let t = trybuild::TestCases::new();
    // t.pass("tests/html_macro_tests/literals/single_literal_pass.rs");
    // t.compile_fail("tests/html_macro_tests/literals/single_literal_fail.rs");
    // t.pass("tests/html_macro_tests/expressions/single_expression_pass.rs");
    // t.compile_fail("tests/html_macro_tests/expressions/single_expression_fail.rs");
    // t.pass("tests/html_macro_tests/elements/single_element_pass.rs");
    // t.pass("tests/html_macro_tests/elements/multiple_elements_pass.rs");
    // t.pass("tests/html_macro_tests/elements/element_with_single_literal_pass.rs");
    // t.compile_fail("tests/html_macro_tests/elements/element_with_single_literal_fail.rs");
    // t.pass("tests/html_macro_tests/elements/element_with_single_expression_pass.rs");
    // t.compile_fail("tests/html_macro_tests/elements/element_with_single_expression_fail.rs");
    // t.pass("tests/html_macro_tests/elements/element_tree_pass.rs");
    // t.pass("tests/html_macro_tests/elements/element_forest_pass.rs");
    // t.compile_fail("tests/html_macro_tests/elements/element_without_closing_fail.rs");
    // t.compile_fail("tests/html_macro_tests/elements/element_without_opening_fail.rs");
    // t.compile_fail("tests/html_macro_tests/elements/element_with_wrong_way_of_closing_fail.rs");
    // t.pass("tests/html_macro_tests/attributes/attributes_pass.rs");
    // t.compile_fail("tests/html_macro_tests/attributes/attributes_fail.rs");
    // t.pass("tests/html_macro_tests/fragments/fragment_pass.rs");
    // t.compile_fail("tests/html_macro_tests/fragments/fragment_fail.rs");
    t.pass("tests/html_macro_tests/for/for_loop_pass.rs");
    // t.compile_fail("tests/html_macro_tests/for/for_loop_fail.rs");
    // t.pass("tests/html_macro_tests/ifs/if_pass.rs");
    // t.compile_fail("tests/html_macro_tests/ifs/if_fail.rs");
    // t.pass("tests/html_macro_tests/ifs/if_let_pass.rs");
    // t.compile_fail("tests/html_macro_tests/ifs/if_let_fail.rs");
    // t.pass("tests/html_macro_tests/ifs/if_and_if_let_pass.rs");
}
