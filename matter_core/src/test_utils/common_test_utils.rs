use float_cmp::ApproxEq;
pub fn assert_float(result: f64, expected: f64) {
    assert!(
        result.approx_eq(expected, (0.0, 2)),
        "result: {} did not match expected: {}",
        result,
        expected
    );
}
