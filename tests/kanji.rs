use cjk::*;

#[test]
fn kanji1() {
   assert_eq!(JOUYOU_TABLE.len(), 2136);
}

#[test]
fn kanji2() {
   assert_eq!(JOUYOU_PRONUNCIATION_INDEX.len(), 1114);
}
