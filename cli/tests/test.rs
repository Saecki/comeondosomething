use assert_cmd::Command;

#[test]
fn dyn_capture() {
    let input = "\
fn outer() {
    let a = 3

    fn inner() -> int {
        a
    }
}
";
    let output = "\
[1;94m2 │[0m     let a = 3
  [1;94m│[0m         [1;91m^[0m
[1;94m3 │[0m 
[1;94m4 │[0m     fn inner() -> int {
[1;94m5 │[0m         a
  [1;94m│[0m         [1;91m^[0m
  [1;94m│[0m [1;91mCapturing variables from a dynamic scope is not yet implemented[0m[0m

";

    Command::cargo_bin("cods")
        .unwrap()
        .arg("--")
        .arg(input)
        .assert()
        .failure()
        .stdout(output);
}

#[test]
fn mark_error_after_line_end() {
    let input = "4 -";
    let output = "\
[1;94m1 │[0m 4 -
  [1;94m│[0m    [1;91m^[0m
  [1;94m│[0m [1;91mMissing operand[0m[0m

";

    Command::cargo_bin("cods")
        .unwrap()
        .arg("--")
        .arg(input)
        .assert()
        .failure()
        .stdout(output);
}

#[test]
fn builtin_fun_signature() {
    let input = "clamp('a', false, 3)";
    let output = "\
[1;94m1 │[0m clamp('a', false, 3)
  [1;94m│[0m [1;91m^^^^^^^^^^^^^^^^^^^^[0m
  [1;94m│[0m [1;91mNo matching signature for builtin function `clamp`:[0m
  [1;94m│[0m [1;91m    clamp(int, int, int) -> int[0m
  [1;94m│[0m [1;91m    clamp(float, float, float) -> float[0m
  [1;94m│[0m [1;91m[0m
  [1;94m│[0m [1;91mCalled with args of type:[0m
  [1;94m│[0m [1;91m    clamp(char, bool, int)[0m
[0m[0m

";

    Command::cargo_bin("cods")
        .unwrap()
        .arg("--")
        .arg(input)
        .assert()
        .failure()
        .stdout(output);
}
