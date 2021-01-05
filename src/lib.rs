use std::collections::{HashMap,HashSet};
use lazy_static::lazy_static;
use widestring::U32String;

pub static HIRAGANA_TO_ROMAJI_DATA: [(char,&str); 83] = [
   ('あ',"a"),  ('い',"i"),   ('う',"u"),   ('え',"e"),  ('お',"o"),
   ('か',"ka"), ('き',"ki"),  ('く',"ku"),  ('け',"ke"), ('こ',"ko"),
   ('が',"ga"),	('ぎ',"gi"),  ('ぐ',"gu"),  ('げ',"ge"), ('ご',"go"),
   ('さ',"sa"),	('し',"shi"), ('す',"su"),  ('せ',"se"), ('そ',"so"),
   ('ざ',"za"),	('じ',"ji"),  ('ず',"zu"),  ('ぜ',"ze"), ('ぞ',"zo"),
   ('た',"ta"),	('ち',"chi"), ('つ',"tsu"), ('て',"te"), ('と',"to"),
   ('だ',"da"),	('ぢ',"di"),  ('づ',"du"),  ('で',"de"), ('ど',"do"),
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
   pub static ref HIRAGANA_TO_ROMAJI: HashMap<char,String> = {
      let mut ktr = HashMap::new();
      for (c,r) in HIRAGANA_TO_ROMAJI_DATA.iter() {
         ktr.insert(*c,r.to_string());
      }
      ktr
   };
   pub static ref HIRAGANA: HashSet<char> = {
      let mut index = HashSet::new();
      for (c,_) in HIRAGANA_TO_ROMAJI_DATA.iter() {
         index.insert(*c);
      }
      index
   };
}

pub static KATAKANA_TO_ROMAJI_DATA: [(char,&str); 85] = [
   ('ア',"a"),  ('イ',"i"),   ('ウ',"u"),   ('エ',"e"),	 ('オ',"o"),
   ('カ',"ka"), ('キ',"ki"),  ('ク',"ku"),  ('ケ',"ke"), ('コ',"ko"),
   ('ガ',"ga"), ('ギ',"gi"),  ('グ',"gu"),  ('ゲ',"ge"), ('ゴ',"go"),
   ('サ',"sa"), ('シ',"shi"), ('ス',"su"),  ('セ',"se"), ('ソ',"so"),
   ('ザ',"za"), ('ジ',"ji"),  ('ズ',"zu"),  ('ゼ',"ze"), ('ゾ',"zo"),
   ('タ',"ta"), ('チ',"chi"), ('ツ',"tsu"), ('テ',"te"), ('ト',"to"),
   ('ダ',"da"), ('ヂ',"di"),  ('ヅ',"du"),  ('デ',"de"), ('ド',"do"),
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
   pub static ref KATAKANA_TO_ROMAJI: HashMap<char,String> = {
      let mut ktr = HashMap::new();
      for (c,r) in KATAKANA_TO_ROMAJI_DATA.iter() {
         ktr.insert(*c,r.to_string());
      }
      ktr
   };
   pub static ref KATAKANA: HashSet<char> = {
      let mut index = HashSet::new();
      for (c,_) in KATAKANA_TO_ROMAJI_DATA.iter() {
         index.insert(*c);
      }
      index
   };
}

pub static HALFWIDTH_KATAKANA_TO_ROMAJI_DATA: [(char,&str); 57] = [
   //dakuten and handakuten make two code points, not one char
   ('ｱ',"a"),  ('ｲ',"i"),   ('ｳ',"u"),   ('ｴ',"e"),	 ('ｵ',"o"),
   ('ｶ',"ka"), ('ｷ',"ki"),  ('ｸ',"ku"),  ('ｹ',"ke"), ('ｺ',"ko"),
   ('ｻ',"sa"), ('ｼ',"shi"), ('ｽ',"su"),  ('ｾ',"se"), ('ｿ',"so"),
   ('ﾀ',"ta"), ('ﾁ',"chi"), ('ﾂ',"tsu"), ('ﾃ',"te"), ('ﾄ',"to"),
   ('ﾅ',"na"), ('ﾆ',"ni"),  ('ﾇ',"nu"),  ('ﾈ',"ne"), ('ﾉ',"no"),
   ('ﾊ',"ha"), ('ﾋ',"hi"),  ('ﾌ',"fu"),  ('ﾍ',"he"), ('ﾎ',"ho"),
   ('ﾏ',"ma"), ('ﾐ',"mi"),  ('ﾑ',"mu"),  ('ﾒ',"me"), ('ﾓ',"mo"),
   ('ﾔ',"ya"),		      ('ﾕ',"yu"),		 ('ﾖ',"yo"),
   ('ﾗ',"ra"), ('ﾘ',"ri"),  ('ﾙ',"ru"),  ('ﾚ',"re"), ('ﾛ',"ro"),
   ('ﾜ',"wa"),		     ('ｦ',"wo"),
   ('ﾝ',"n"),
   ('ｯ',"xtsu"),
   ('ｬ',"xya"),              ('ｭ',"xyu"),              ('ｮ',"xyo"),
   ('ｧ',"xa"), ('ｨ',"xi"),  ('ｩ',"xu"),  ('ｪ',"xe"), ('ｫ',"xo"),
   ('ヵ',"xka"), ('ヶ',"xke")
];
lazy_static! {
   pub static ref HALFWIDTH_KATAKANA_TO_ROMAJI: HashMap<char,String> = {
      let mut ktr = HashMap::new();
      for (c,r) in HALFWIDTH_KATAKANA_TO_ROMAJI_DATA.iter() {
         ktr.insert(*c,r.to_string());
      }
      ktr
   };
   pub static ref HALFWIDTH_KATAKANA: HashSet<char> = {
      let mut index = HashSet::new();
      for (c,_) in HALFWIDTH_KATAKANA_TO_ROMAJI_DATA.iter() {
         index.insert(*c);
      }
      index
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
               let pr = to_romaji(pr);
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
   pub static ref JOUYOU_KANJI: HashSet<char> = {
      let mut index: HashSet<char> = HashSet::new();
      for (c,_) in JOUYOU_ATEJI_INDEX.iter() {
         index.insert(*c);
      }
      index
   };
   pub static ref JAPANESE_PUNCTUATION: HashSet<char> = {
      let mut index = HashSet::new();
      for c in "⟨　⟩ ー、，…‥。｛　｝（　）［　］【　】〽「　」『　』〝　〟〜：！？♪".chars() {
         index.insert(c);
      }
      index
   };
   pub static ref JAPANESE: HashSet<char> = {
      let mut index = HashSet::new();
      for c in HIRAGANA.iter() {
         index.insert(*c);
      }
      for c in KATAKANA.iter() {
         index.insert(*c);
      }
      for c in HALFWIDTH_KATAKANA.iter() {
         index.insert(*c);
      }
      for c in JOUYOU_KANJI.iter() {
         index.insert(*c);
      }
      for c in JAPANESE_PUNCTUATION.iter() {
         index.insert(*c);
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
   pub point: char,
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
      let mut index: HashMap<char,UnihanCharacter> = HashMap::new();
      for line in include_str!("../unihan_data/Unihan_RadicalStrokeCounts.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         let code = vs[0];
         let code_char = decode_unicode_32(code).chars().next().unwrap_or(' ');
         if vs[1]=="kRSAdobe_Japan1_6" {
            let mut radicals = Vec::new();
            for index in vs[2].split(' ') {
               let mut adobe = index.split('+');
               let canonical = adobe.next().unwrap();
               let _cid = adobe.next().unwrap();
               let radical = adobe.next().unwrap();
               let mut radical_parts = radical.split('.');
               let radical_index = radical_parts.next().unwrap().parse::<u64>().unwrap();
               let radical_stroke_count = radical_parts.next().unwrap().parse::<u64>().unwrap();
               let remainder_stroke_count = radical_parts.next().unwrap().parse::<u64>().unwrap();
               radicals.push(UnihanRadicalStrokeCount {
                  radical: radical_index,
                  canonical: canonical=="C",
                  radical_stroke_count: radical_stroke_count,
                  remainder_stroke_count: remainder_stroke_count,
               });
            }
            index.insert(code_char, UnihanCharacter {
               point: code_char,
               radicals: radicals,
            });
         } else if vs[1]=="kRSKangXi" {
         } else {
            unreachable!();
         }
      }
      for line in include_str!("../unihan_data/Unihan_RadicalStrokeCounts.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         let code = vs[0];
         let code_char = decode_unicode_32(code).chars().next().unwrap_or(' ');
         if vs[1]=="kRSAdobe_Japan1_6" {
         } else if vs[1]=="kRSKangXi" {
            if !index.contains_key(&code_char) {
               index.insert(code_char, UnihanCharacter {
                  point: code_char,
                  radicals: Vec::new(),
               });
            }
            let ref mut rs = index.get_mut(&code_char).unwrap().radicals;
            let mut radical_index = vs[2].split('.');
            let radical = radical_index.next().unwrap().parse::<u64>().unwrap();
            let remainder = radical_index.next().unwrap().parse::<u64>().unwrap_or(0);
            if rs.iter().any(|r| r.radical==radical && r.remainder_stroke_count==remainder) { continue; }
            rs.push(UnihanRadicalStrokeCount {
               radical: radical,
               canonical: false,
               radical_stroke_count: 0,
               remainder_stroke_count: remainder,
            });
         } else {
            unreachable!();
         }
      }
      let mut rsc = HashMap::new();
      rsc.insert(&52, 3); //乡 3
      rsc.insert(&102, 5); //曱 5
      for (ri,r) in UNIHAN_RADICALS.iter() {
         //if index.get(&r.point).unwrap().radicals.iter().all(|rr| rr.radical_stroke_count==0) {
         //   println!("no radical count for {} #{}", r.point, ri);
         //}
         for rr in index.get(&r.point).unwrap().radicals.iter() {
            if rr.radical_stroke_count==0 { continue; }
            rsc.insert(ri, rr.radical_stroke_count);
            break;
         }
      }
      for (_,ch) in index.iter_mut() {
         for r in ch.radicals.iter_mut() {
            if r.radical_stroke_count == 0 {
               r.radical_stroke_count = *rsc.get(&r.radical).unwrap();
            }
         }
      }
      index
   };
   pub static ref UNIHAN_ANY_VARIANT: HashMap<char,HashSet<char>> = {
      let mut index: HashMap<char,HashSet<char>> = HashMap::new();
      for line in include_str!("../unihan_data/Unihan_Variants.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         let code = vs[0];
         if let Some(code_char) = decode_unicode_32(code).chars().next() {
            if !index.contains_key(&code_char) {
               index.insert(code_char, HashSet::new());
            }
            for var in vs[2].split_whitespace() {
               let cvar = var.split('<').next().unwrap();
               let cvar = decode_unicode_32(cvar).chars().next().unwrap();
               if !index.contains_key(&cvar) {
                  index.insert(cvar, HashSet::new());
               }

               index.get_mut(&code_char).unwrap().insert(cvar);
               index.get_mut(&cvar).unwrap().insert(code_char);
            }
         }
      }
      index
   };
   pub static ref UNIHAN_SIMPLIFIED_CHINESE: HashSet<char> = {
/*
#       kSemanticVariant
#       kSimplifiedVariant
#       kSpecializedSemanticVariant
#       kSpoofingVariant
#       kTraditionalVariant
#       kZVariant
*/
      let mut index = HashSet::new();
      for line in include_str!("../unihan_data/Unihan_Variants.txt").split('\n') {
         if line.len()==0 { continue; }
         if &line[0..1]=="#" { continue; }
         let vs = line.split('\t').collect::<Vec<&str>>();
         let code = vs[0];
         let code_char = decode_unicode_32(code).chars().next().unwrap_or(' ');
         if vs[1]=="kTraditionalVariant" {
            index.insert(code_char);
         }
      }
      index
   };
}

/// <b>to_romaji</b> attempts to convert Japanese text to romaji.
pub fn to_romaji(s: &str) -> String {
   let mut o = String::new();
   for c in s.chars() {
      if let Some(r) = HIRAGANA_TO_ROMAJI.get(&c) {
         o.push_str(&r);
      } else if let Some(r) = KATAKANA_TO_ROMAJI.get(&c) {
         o.push_str(&r);
      } else if let Some(r) = HALFWIDTH_KATAKANA_TO_ROMAJI.get(&c) {
         o.push_str(&r);
      } else {
         o.push(c);
      }
   }
   o
}

/// <b>to_pinyin</b> attempts to convert Chinese text to pinyin.
pub fn to_pinyin(s: &str) -> String {
   let _ = s;
   unimplemented!("to_pinyin has not been implemented")
}

/// <b>to_hangul</b> attempts to convert mixed Korean text to hangul.
pub fn to_hangul(s: &str) -> String {
   let _ = s;
   unimplemented!("to_hangul has not been implemented")
}

/// <b>get_stroke_count</b> returns the number of strokes 
/// typically used to write the given character.
pub fn get_stroke_count(c: char) -> u64 {
   let _ = c;
   unimplemented!("stroke_count has not been implemented")
}

/// <b>get_radical</b> returns the Kangxi numerical index of
/// the given character.
pub fn get_radical(c: char) -> Option<u64> {
   //dictionary-order radical
   if let Some(ch) = UNIHAN_CHARACTERS.get(&c) {
      ch.radicals.iter().filter(|r| r.canonical).map(|r| r.radical).next()
   } else {
      None
   }
}

/// <b>get_radicals</b> returns the Kangxi numerical indexes of
/// all radicals or their variants present in the given character.
pub fn get_radicals(c: char) -> Vec<u64> {
   //any radical in character
   if let Some(ch) = UNIHAN_CHARACTERS.get(&c) {
      ch.radicals.iter().filter(|r| r.canonical).map(|r| r.radical).collect::<Vec<u64>>()
   } else {
      Vec::new()
   }
}

/// <b>get_parts</b> returns all radicals or their variants
/// present in the given character.
pub fn get_parts(c: char) -> Vec<char> {
   //any part or radical in character
   let _  = c;
   unimplemented!("parts has not been implemented")
}

/// <b>get_variants</b> returns all equivalent variants
/// of the given character.
pub fn get_variants(c: char) -> Vec<char> {
   if let Some(cvs) = UNIHAN_ANY_VARIANT.get(&c) {
      let mut cvs = cvs.iter().map(|c| *c).collect::<Vec<char>>();
      cvs.sort();
      cvs
   } else {
      Vec::new()
   }
}

/// <b>is_traditional_chinese</b> returns true if the string can
/// almost certainly be read as traditional chinese.
pub fn is_traditional_chinese(s: &str) -> bool {
   s.chars().all(|c| is_cjkish_codepoint(c))
}

/// <b>is_simplified_chinese</b> returns true if the string can
/// almost certainly be read as simplified chinese.
pub fn is_simplified_chinese(s: &str) -> bool {
   s.chars().all(|c| is_cjkish_codepoint(c))
}

/// <b>is_japanese</b> returns true if the string can
/// almost certainly be read as japanese.
pub fn is_japanese(s: &str) -> bool {
   s.chars().all(|c| is_cjkish_codepoint(c))
}

/// <b>is_korean</b> returns true if the string can
/// almost certainly be read as korean.
pub fn is_korean(s: &str) -> bool {
   s.chars().all(|c| is_cjkish_codepoint(c))
}

/// <b>is_vietnamese</b> returns true if the string can
/// almost certainly be read as vietnamese.
pub fn is_vietnamese(s: &str) -> bool {
   s.chars().all(|c| is_cjkish_codepoint(c))
}

/// <b>is_cjk_codepoint</b> returns true if the character falls
/// within the CJK unicode block. The CJK unicode block does not
/// contain all chinese, japanese, korean, or vietnamese characters.
///
/// Despite this shortcoming, this utility is widely used and
/// we provide it here for applications that expect it.
pub fn is_cjk_codepoint(c: char) -> bool {
   let cp: u32 = c.into();
   (cp >= 0x4E00 && cp <= 0x9FFF) ||
   (cp >= 0x3400 && cp <= 0x4DBF) ||
   (cp >= 0x20000 && cp <= 0x2A6DF) ||
   (cp >= 0x2A700 && cp <= 0x2B73F) ||
   (cp >= 0x2B740 && cp <= 0x2B81F) ||
   (cp >= 0x2B820 && cp <= 0x2CEAF) ||
   (cp >= 0xF900 && cp <= 0xFAFF) ||
   (cp >= 0x2F800 && cp <= 0x2FA1F)
}

/// <b>is_cjk_punctuation_codepoint</b> returns true if the character falls
/// within the CJK punctuation unicode block. The CJK punctuation unicode block does not
/// contain all chinese, japanese, korean, or vietnamese characters.
pub fn is_cjk_punctuation_codepoint(c: char) -> bool {
   let cp: u32 = c.into();
   (cp >= 0x0000 && cp <= 0x009F) ||
   (cp >= 0x3000 && cp <= 0x303F) ||
   (cp >= 0xFF00 && cp <= 0xFF9F)
}

/// <b>is_cjkish_codepoint</b> returns true if the character falls
/// within the CJK or related unicode blocks. The CJK-ish unicode blocks do
/// contain most chinese, japanese, korean, or vietnamese characters.
/// However this comes at the price of specificity and contains many potentially
/// illegible codepoints.
pub fn is_cjkish_codepoint(c: char) -> bool {
   is_cjk_codepoint(c) ||
   is_cjk_punctuation_codepoint(c)
}
