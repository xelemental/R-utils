use assert_cmd::Command;

#[test]
fn test_works() -> (){
let mut spawnnewcommand = Command::cargo_bin("truebinary").unwrap();
spawnnewcommand.assert().success();
}
