use cjk::romaji;

#[test]
fn kana1() {
   assert_eq!(romaji("バナナ"), "banana");
}

#[test]
fn kana2() {
   assert_eq!(romaji("やなぎ"), "yanagi");
}
