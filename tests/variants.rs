

#[test]
fn variants1() {
   assert_eq!(cjk::UNIHAN_ANY_VARIANT.len(), 14957);
}

#[test]
fn variants2() {
   assert_eq!(cjk::variants('儒'), vec!['㐵']);
   assert_eq!(cjk::variants('㐵'), vec!['儒']);
}

#[test]
fn variants3() {
   assert_eq!(cjk::variants(' '), vec![]);
}
