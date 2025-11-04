#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    // Positive cases should compile
    t.pass("tests/ui/positive.rs");
    // Negative cases should fail with expected errors
    t.compile_fail("tests/ui/negative_missing_fields.rs");
    t.compile_fail("tests/ui/negative_wrong_types.rs");
}
