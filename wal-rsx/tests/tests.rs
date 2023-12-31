#[test]
fn rsx_macro_tests() {
    let t = trybuild::TestCases::new();
    // t.pass("tests/literals/single_literal_pass.rs");
    // t.compile_fail("tests/literals/single_literal_fail.rs");
    // t.pass("tests/expressions/single_expression_pass.rs");
    // t.compile_fail("tests/expressions/single_expression_fail.rs");
    // t.pass("tests/elements/single_element_pass.rs");
    // t.pass("tests/elements/multiple_elements_pass.rs");
    // t.pass("tests/elements/element_with_single_literal_pass.rs");
    // t.compile_fail("tests/elements/element_with_single_literal_fail.rs");
    // t.pass("tests/elements/element_with_single_expression_pass.rs");
    // t.compile_fail("tests/elements/element_with_single_expression_fail.rs");
    // t.pass("tests/elements/element_tree_pass.rs");
    // t.pass("tests/elements/element_forest_pass.rs");
    // t.compile_fail("tests/elements/element_without_closing_fail.rs");
    // t.compile_fail("tests/elements/element_without_opening_fail.rs");
    // t.compile_fail("tests/elements/element_with_wrong_way_of_closing_fail.rs");
    // t.pass("tests/attributes/attributes_pass.rs");
    // t.compile_fail("tests/attributes/attributes_fail.rs");
    // t.pass("tests/attributes/event_attributes_pass.rs");
    // t.compile_fail("tests/attributes/event_attributes_fail.rs");
    // t.pass("tests/attributes/class_attributes_pass.rs");
    // t.compile_fail("tests/attributes/class_attributes_fail.rs");
    t.pass("tests/attributes/attributes_with_names_same_as_rusts_keywords_pass.rs");
    // t.pass("tests/fragments/fragment_pass.rs");
    // t.compile_fail("tests/fragments/fragment_fail.rs");
    // t.pass("tests/for/for_loop_pass.rs");
    // t.compile_fail("tests/for/for_loop_fail.rs");
    // t.pass("tests/ifs/if_pass.rs");
    // t.compile_fail("tests/ifs/if_fail.rs");
    // t.pass("tests/ifs/if_let_pass.rs");
    // t.compile_fail("tests/ifs/if_let_fail.rs");
    // t.pass("tests/ifs/if_and_if_let_pass.rs");
    // t.pass("tests/ifs/if_should_return_vlist_pass.rs");
    // t.pass("tests/custom_components/custom_component_without_props_pass.rs");
    // t.compile_fail(
    //     "tests/custom_components/custom_component_without_props_without_default_fail.rs",
    // );
    // t.compile_fail("tests/custom_components/custom_component_without_props_without_hash_fail.rs");
    // t.pass("tests/custom_components/custom_component_with_props_pass.rs");
    // t.compile_fail("tests/custom_components/custom_component_wrong_attributes_fail.rs");
    // t.pass("tests/links/link_should_pass.rs");
    // t.compile_fail("tests/links/link_should_fail.rs");
}
