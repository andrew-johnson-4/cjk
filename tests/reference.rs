
#[test]
fn reference1() {
    assert_eq!(cjk::radical('待'), Some(60));
}

#[test]
fn reference2() {
    assert_eq!(cjk::radicals('待'), vec![60]);
}

/*
#[test]
fn reference3() {
    assert_eq!(cjk::parts('待'), vec!['土','寸','彳']);
}
*/
