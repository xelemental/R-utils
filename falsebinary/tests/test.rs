use assert_cmd::Command;


#[test]
fn check_false(){
let mut newcommand = Command::cargo_bin("falsebinary").unwrap();
newcommand.assert().failure();
}
