use cjk::to_romaji;

#[test]
fn kana1() {
   assert_eq!(to_romaji("バナナ"), "banana");
}

#[test]
fn kana2() {
   assert_eq!(to_romaji("やなぎ"), "yanagi");
}
