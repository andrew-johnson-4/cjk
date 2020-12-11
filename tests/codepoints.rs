use cjk::*;

#[test]
fn char1() {
   assert!(!is_cjk_codepoint('a'));
}
