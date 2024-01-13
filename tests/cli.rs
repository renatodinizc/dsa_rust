use assert_cmd::Command;

#[test]
fn assert_bubble_sort() {
    let mut cmd = Command::cargo_bin("dsa_rust").unwrap();

    cmd.args(["-b", "123", "45", "3", "28", "74", "19123", "28", "28", "1"]);

    cmd.assert()
        .success()
        .stdout("[1, 3, 28, 28, 28, 45, 74, 123, 19123]\n");
}

#[test]
fn assert_bubble_sort2() {
    let mut cmd = Command::cargo_bin("dsa_rust").unwrap();

    cmd.args(["-b", "1"]);

    cmd.assert().success().stdout("[1]\n");
}

#[test]
fn assert_selection_sort() {
    let mut cmd = Command::cargo_bin("dsa_rust").unwrap();

    cmd.args(["-s", "123", "45", "3", "28", "74", "19123", "28", "28", "1"]);

    cmd.assert()
        .success()
        .stdout("[1, 3, 28, 28, 28, 45, 74, 123, 19123]\n");
}

#[test]
fn assert_selection_sort2() {
    let mut cmd = Command::cargo_bin("dsa_rust").unwrap();

    cmd.args(["-s", "1"]);

    cmd.assert().success().stdout("[1]\n");
}

#[test]
fn assert_insertion_sort() {
    let mut cmd = Command::cargo_bin("dsa_rust").unwrap();

    cmd.args(["-i", "123", "45", "3", "28", "74", "19123", "28", "28", "1"]);

    cmd.assert()
        .success()
        .stdout("[1, 3, 28, 28, 28, 45, 74, 123, 19123]\n");
}

#[test]
fn assert_insertion_sort2() {
    let mut cmd = Command::cargo_bin("dsa_rust").unwrap();

    cmd.args(["-i", "1"]);

    cmd.assert().success().stdout("[1]\n");
}
