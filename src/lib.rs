use std::collections::{HashMap};
use lazy_static::lazy_static;

static HIRAGANA_TO_ROMAJI: [(char,&str); 72] = [
   ('あ',"a"),  ('い',"i"),   ('う',"u"),   ('え',"e"),  ('お',"o"),
   ('か',"ka"), ('き',"ki"),  ('く',"ku"),  ('け',"ke"), ('こ',"ko"),
   ('が',"ga"),	('ぎ',"gi"),  ('ぐ',"gu"),  ('げ',"ge"), ('ご',"go"),
   ('さ',"sa"),	('し',"shi"), ('す',"su"),  ('せ',"se"), ('そ',"so"),
   ('ざ',"za"),	('じ',"ji"),  ('ず',"zu"),  ('ぜ',"ze"), ('ぞ',"zo"),
   ('た',"ta"),	('ち',"chi"), ('つ',"tsu"), ('て',"te"), ('と',"to"),
   ('だ',"da"),	('ぢ',"ji"),  ('づ',"zu"),  ('で',"de"), ('ど',"do"),
   ('な',"na"),	('に',"ni"),  ('ぬ',"nu"),  ('ね',"ne"), ('の',"no"),
   ('は',"ha"),	('ひ',"hi"),  ('ふ',"fu"),  ('へ',"he"), ('ほ',"ho"),
   ('ば',"ba"),	('び',"bi"),  ('ぶ',"bu"),  ('べ',"be"), ('ぼ',"bo"),
   ('ぱ',"pa"),	('ぴ',"pi"),  ('ぷ',"pu"),  ('ぺ',"pe"), ('ぽ',"po"),
   ('ま',"ma"),	('み',"mi"),  ('む',"mu"),  ('め',"me"), ('も',"mo"),
   ('や',"ya"),	              ('ゆ',"yu"),               ('よ',"yo"),
   ('ら',"ra"),	('り',"ri"),  ('る',"ru"),  ('れ',"re"), ('ろ',"ro"),
   ('わ',"wa"),	('ゐ',"wi"),                ('ゑ',"we"), ('を',"wo"),
];
lazy_static! {
   pub static ref H2R: HashMap<char,String> = {
      let mut ktr = HashMap::new();
      for (c,r) in HIRAGANA_TO_ROMAJI.iter() {
         ktr.insert(*c,r.to_string());
      }
      ktr
   };
}

pub fn romaji(s: &str) -> String {
   let mut o = String::new();
   for c in s.chars() {
      if let Some(r) = H2R.get(&c) {
         o.push_str(&r);
      } else {
         o.push(c);
      }
   }
   o
}
