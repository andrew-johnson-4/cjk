use cjk::*;
use std::path::PathBuf;

#[test]
fn chinese_wot() {
   let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
   d.push("tests/data/wall_of_text_zh.txt");
   let chinese_wot = std::fs::read_to_string(d).expect("chinese wall of text");
   if !is_simplified_chinese(&chinese_wot) {
      for c in chinese_wot.chars() {
         if !is_simplified_chinese(&format!("{}",c)) {
            panic!("'{}': {} is not simplified chinese", c, c as u32)
         }
      }
   }
}

#[test]
fn japanese_wot() {
   let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
   d.push("tests/data/wall_of_text_ja.txt");
   let japanese_wot = std::fs::read_to_string(d).expect("japanese wall of text");
   if !is_japanese(&japanese_wot) {
      for c in japanese_wot.chars() {
         if !is_japanese(&format!("{}",c)) {
            panic!("'{}': {} is not japanese", c, c as u32)
         }
      }
   }
}

#[test]
fn korean_wot() {
   let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
   d.push("tests/data/wall_of_text_ko.txt");
   let korean_wot = std::fs::read_to_string(d).expect("korean wall of text");
   if !is_korean(&korean_wot) {
      for c in korean_wot.chars() {
         if !is_korean(&format!("{}",c)) {
            panic!("'{}': {} is not korean", c, c as u32)
         }
      }
   }
}
