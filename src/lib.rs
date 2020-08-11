use std::collections::{HashMap};
use lazy_static::lazy_static;
use widestring::U32String;

pub static HIRAGANA_TO_ROMAJI: [(char,&str); 83] = [
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
   ('ん',"n"),
   //romanization of these characters is context sensitive, especially for aeiou
   //here we prefix with x for later consumption
   ('ゔ',"v"),
   ('っ',"xtsu"),
   ('ゃ',"xya"),              ('ゅ',"xyu"),              ('ょ',"xyo"),
   ('ぁ',"xa"), ('ぃ',"xi"),  ('ぅ',"xu"),  ('ぇ',"xe"), ('ぉ',"xo"),
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

pub static KATAKANA_TO_ROMAJI: [(char,&str); 85] = [
   ('ア',"a"),  ('イ',"i"),   ('ウ',"u"),   ('エ',"e"),	 ('オ',"o"),
   ('カ',"ka"), ('キ',"ki"),  ('ク',"ku"),  ('ケ',"ke"), ('コ',"ko"),
   ('ガ',"ga"), ('ギ',"gi"),  ('グ',"gu"),  ('ゲ',"ge"), ('ゴ',"go"),
   ('サ',"sa"), ('シ',"shi"), ('ス',"su"),  ('セ',"se"), ('ソ',"so"),
   ('ザ',"za"), ('ジ',"ji"),  ('ズ',"zu"),  ('ゼ',"ze"), ('ゾ',"zo"),
   ('タ',"ta"), ('チ',"chi"), ('ツ',"tsu"), ('テ',"te"), ('ト',"to"),
   ('ダ',"da"), ('ヂ',"ji"),  ('ヅ',"zu"),  ('デ',"de"), ('ド',"do"),
   ('ナ',"na"), ('ニ',"ni"),  ('ヌ',"nu"),  ('ネ',"ne"), ('ノ',"no"),
   ('ハ',"ha"), ('ヒ',"hi"),  ('フ',"fu"),  ('ヘ',"he"), ('ホ',"ho"),
   ('バ',"ba"), ('ビ',"bi"),  ('ブ',"bu"),  ('ベ',"be"), ('ボ',"bo"),
   ('パ',"pa"), ('ピ',"pi"),  ('プ',"pu"),  ('ペ',"pe"), ('ポ',"po"),
   ('マ',"ma"), ('ミ',"mi"),  ('ム',"mu"),  ('メ',"me"), ('モ',"mo"),
   ('ヤ',"ya"),		      ('ユ',"yu"),		 ('ヨ',"yo"),
   ('ラ',"ra"), ('リ',"ri"),  ('ル',"ru"),  ('レ',"re"), ('ロ',"ro"),
   ('ワ',"wa"), ('ヰ',"wi"),		    ('ヱ',"we"), ('ヲ',"wo"),
   ('ン',"n"),
   ('ヴ',"v"),
   ('ッ',"xtsu"),
   ('ャ',"xya"),              ('ュ',"xyu"),              ('ョ',"xyo"),
   ('ァ',"xa"), ('ィ',"xi"),  ('ゥ',"xu"),  ('ェ',"xe"), ('ォ',"xo"),
   ('ヵ',"xka"), ('ヶ',"xke")
];
lazy_static! {
   pub static ref K2R: HashMap<char,String> = {
      let mut ktr = HashMap::new();
      for (c,r) in KATAKANA_TO_ROMAJI.iter() {
         ktr.insert(*c,r.to_string());
      }
      ktr
   };
}

#[derive(Clone)]
pub struct JouyouRecord {
   pub number: u64,
   pub new: char,
   pub old: Option<char>,
   pub radical: Option<char>,
   pub strokes: u64,
   pub grade: String,
   pub year: Option<u64>,
   pub translation: String,
   pub pronunciation: Vec<String>,
}

lazy_static! {
   pub static ref JOUYOU_TABLE: Vec<JouyouRecord> = {
      let mut ks = Vec::new();
      for line in include_str!("../wikipedia_data/jouyou.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         ks.push(JouyouRecord {
            number: vs[0].parse::<u64>().expect("number"),
            new: vs[1].chars().next().unwrap(),
            old: vs[2].chars().next(),
            radical: vs[3].chars().next(),
            strokes: vs[4].parse::<u64>().expect("strokes"),
            grade: vs[5].to_string(),
            year: vs[6].parse::<u64>().ok(),
            translation: vs[7].to_string(),
            pronunciation: vs[8].split('、').map(|s| s.to_string()).collect::<Vec<String>>()
         });
      }
      ks
   };
   pub static ref JOUYOU_PRONUNCIATION_INDEX: HashMap<String,Vec<JouyouRecord>> = {
      let mut index: HashMap<String,Vec<JouyouRecord>> = HashMap::new();
      for jr in JOUYOU_TABLE.iter() {
         for p in jr.pronunciation.iter() {
            if let Some(pr) = p.split('-').next() {
               let pr = romaji(pr);
               if !index.contains_key(&pr) {
                  index.insert(pr.clone(), Vec::new());
               }
               if let Some(ps) = index.get_mut(&pr) {
                  if !ps.iter().any(|pjr| (*pjr).number==jr.number) {
                     ps.push(jr.clone());
                  }
               }
            }
         }
      }
      index
   };
   pub static ref JOUYOU_ATEJI_INDEX: HashMap<char,Vec<char>> = {
      let mut index: HashMap<char,Vec<char>> = HashMap::new();
      for (_,ts) in JOUYOU_PRONUNCIATION_INDEX.iter() {
         for ljr in ts.iter() {
            for rjr in ts.iter() {
               if ljr.number==rjr.number { continue; }
               if !index.contains_key(&ljr.new) {
                  index.insert(ljr.new, Vec::new());
               }
               if let Some(cs) = index.get_mut(&ljr.new) {
                  cs.push(rjr.new);
               }
            }
         }
      }
      index
   };
}

pub struct UnihanRadical {
   pub number: u64,
   pub point: char,
   pub variants: Vec<char>,
}
pub struct UnihanCharacter {
   pub stroke_count: u64,
   pub radicals: Vec<UnihanRadicalStrokeCount>,
}
pub struct UnihanRadicalStrokeCount {
   pub radical: u64,
   pub canonical: bool,
   pub radical_stroke_count: u64,
   pub remainder_stroke_count: u64,
}

fn decode_unicode_32(code: &str) -> String {
   let hex_code = if code.len()%2==0 {
      format!("{}", &code[2..])
   } else {
      format!("0{}", &code[2..])
   };
   if let Ok(mut hs) = hex::decode(hex_code) {
      while hs.len()<4 { hs.insert(0,0); }
      let h = hs[0] as u32;
      let h = h*256 + (hs[1] as u32);
      let h = h*256 + (hs[2] as u32);
      let h = h*256 + (hs[3] as u32);
      let wc = U32String::from_vec(vec![h]);
      wc.to_string_lossy()
   } else { "".to_string() }
}

lazy_static! {
   pub static ref UNIHAN_RADICALS: HashMap<u64,UnihanRadical> = {
      let mut index: HashMap<u64,UnihanRadical> = HashMap::new();
      for line in include_str!("../unihan_data/Unihan_RadicalStrokeCounts.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         if vs[1]=="kRSKangXi" {
            let code = vs[0];
            let code_char = decode_unicode_32(code).chars().next().unwrap_or(' ');
            let mut radical_index = vs[2].split('.');
            let radical = radical_index.next().unwrap().parse::<u64>().unwrap();
            let remainder = radical_index.next().unwrap().parse::<i64>().unwrap();
            if remainder==0 {
               if !index.contains_key(&radical) {
                  index.insert(radical, UnihanRadical { number:radical, point:code_char, variants:vec![]});
               }
               let c = index.get_mut(&radical).unwrap();
               c.variants.push(code_char);
            }
         }
      }
      index
   };
   pub static ref UNIHAN_CHARACTERS: HashMap<char,UnihanCharacter> = {
      let index: HashMap<char,UnihanCharacter> = HashMap::new();
      for line in include_str!("../unihan_data/Unihan_RadicalStrokeCounts.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         if vs[1]=="kRSAdobe_Japan1_6" {
            let code = vs[0];
            let code_char = decode_unicode_32(code).chars().next().unwrap_or(' ');
            println!("{} => {}", code_char, line);
         }
      }
      index
   };
}

pub fn romaji(s: &str) -> String {
   let mut o = String::new();
   for c in s.chars() {
      if let Some(r) = H2R.get(&c) {
         o.push_str(&r);
      } else if let Some(r) = K2R.get(&c) {
         o.push_str(&r);
      } else {
         o.push(c);
      }
   }
   o
}

pub fn stroke_count(c: char) -> u64 {
   let _ = c;
   unimplemented!("stroke_count has not been implemented")
}
pub fn radical(c: char) -> Option<char> {
   //dictionary-order radical
   let _ = c;
   unimplemented!("radical has not been implemented")
}
pub fn radicals(c: char) -> Vec<char> {
   //any radical in character
   let _ = c;
   unimplemented!("radicals has not been implemented")
}
pub fn parts(c: char) -> Vec<char> {
   //any part or radical in character
   let _ = c;
   unimplemented!("parts has not been implemented")
}
