
#[test]
fn property1() {
   assert!(cjk::is_traditional_chinese("廣"));
   assert!(cjk::is_traditional_chinese("東"));
   assert!(cjk::is_traditional_chinese("缺"));
   assert!(cjk::is_traditional_chinese("國"));
   assert!(cjk::is_traditional_chinese("界"));
}

#[test]
fn property2() {
   assert!(cjk::is_simplified_chinese("广"));
   assert!(cjk::is_simplified_chinese("东"));
   assert!(cjk::is_simplified_chinese("国"));
}

#[test]
fn property3() {
   assert!(!cjk::is_simplified_chinese("広"));
   assert!(!cjk::is_simplified_chinese("廣"));
}

#[test]
fn property4() {
   assert!(cjk::is_japanese("マラソン五輪代表に五万メートル出場にも含ふくみ"));
   assert!(!cjk::is_japanese("123"));
}
