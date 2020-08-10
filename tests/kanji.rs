use cjk::*;

#[test]
fn kanji1() {
   assert_eq!(JOUYOU_TABLE.len(), 2136);
}

#[test]
fn kanji2() {
   assert_eq!(JOUYOU_PRONUNCIATION_INDEX.len(), 1113);
}

#[test]
fn kanji3() {
   assert_eq!(JOUYOU_ATEJI_INDEX.len(), 2083);
   assert_eq!(JOUYOU_ATEJI_INDEX.get(&'六'), Some(&vec!['緑', '録', '麓', '群', '向', '蒸', '武', '謀', '矛', '務', '無', '夢', '霧']));
}
