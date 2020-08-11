use cjk::*;

#[test]
fn char1() {
   assert_eq!(UNIHAN_CHARACTERS.len(), 64059);
}

#[test]
fn char2() {
   assert!(!UNIHAN_CHARACTERS.iter().any(|(_,r)| r.radicals.len()==0));
}
