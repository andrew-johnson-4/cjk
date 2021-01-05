use cjk::*;
use std::path::PathBuf;

#[test]
fn chinese_wot() {
   let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
   d.push("tests/data/wall_of_text_zh.txt");
   let chinese_wot = std::fs::read_to_string(d).expect("chinese wall of text");
   assert!(is_simplified_chinese(&chinese_wot));
}

#[test]
fn japanese_wot() {
   let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
   d.push("tests/data/wall_of_text_ja.txt");
   let japanese_wot = std::fs::read_to_string(d).expect("japanese wall of text");
   assert!(is_japanese(&japanese_wot));
}

#[test]
fn korean_wot() {
   let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
   d.push("tests/data/wall_of_text_ko.txt");
   let korean_wot = std::fs::read_to_string(d).expect("korean wall of text");
   assert!(is_korean(&korean_wot));
}
