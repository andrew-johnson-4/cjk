use cjk::*;

#[test]
fn radical1() {
   assert_eq!(UNIHAN_RADICALS.len(), 214);
}

#[test]
fn radical2() {
   assert_eq!(UNIHAN_RADICALS.get(&1).unwrap().point, '一'); 
   assert_eq!(UNIHAN_RADICALS.get(&2).unwrap().point, '丨'); 
   assert_eq!(UNIHAN_RADICALS.get(&3).unwrap().point, '丶'); 
   assert_eq!(UNIHAN_RADICALS.get(&4).unwrap().point, '丿'); 
   assert_eq!(UNIHAN_RADICALS.get(&5).unwrap().point, '乙'); 
   assert_eq!(UNIHAN_RADICALS.get(&6).unwrap().point, '亅'); 
   assert_eq!(UNIHAN_RADICALS.get(&7).unwrap().point, '二'); 
   assert_eq!(UNIHAN_RADICALS.get(&8).unwrap().point, '亠'); 
   assert_eq!(UNIHAN_RADICALS.get(&9).unwrap().point, '人'); 
   assert_eq!(UNIHAN_RADICALS.get(&10).unwrap().point, '儿'); 

   assert_eq!(UNIHAN_RADICALS.get(&11).unwrap().point, '入'); 
   assert_eq!(UNIHAN_RADICALS.get(&12).unwrap().point, '八'); 
   assert_eq!(UNIHAN_RADICALS.get(&13).unwrap().point, '冂'); 
   assert_eq!(UNIHAN_RADICALS.get(&14).unwrap().point, '冖'); 
   assert_eq!(UNIHAN_RADICALS.get(&15).unwrap().point, '冫'); 
   assert_eq!(UNIHAN_RADICALS.get(&16).unwrap().point, '几'); 
   assert_eq!(UNIHAN_RADICALS.get(&17).unwrap().point, '凵'); 
   assert_eq!(UNIHAN_RADICALS.get(&18).unwrap().point, '刀'); 
   assert_eq!(UNIHAN_RADICALS.get(&19).unwrap().point, '力'); 
   assert_eq!(UNIHAN_RADICALS.get(&20).unwrap().point, '勹'); 

   assert_eq!(UNIHAN_RADICALS.get(&21).unwrap().point, '匕'); 
   assert_eq!(UNIHAN_RADICALS.get(&22).unwrap().point, '匚'); 
   assert_eq!(UNIHAN_RADICALS.get(&23).unwrap().point, '匸'); 
   assert_eq!(UNIHAN_RADICALS.get(&24).unwrap().point, '十'); 
   assert_eq!(UNIHAN_RADICALS.get(&25).unwrap().point, '卜'); 
   assert_eq!(UNIHAN_RADICALS.get(&26).unwrap().point, '卩'); 
   assert_eq!(UNIHAN_RADICALS.get(&27).unwrap().point, '厂'); 
   assert_eq!(UNIHAN_RADICALS.get(&28).unwrap().point, '厶'); 
   assert_eq!(UNIHAN_RADICALS.get(&29).unwrap().point, '又'); 
   assert_eq!(UNIHAN_RADICALS.get(&30).unwrap().point, '口'); 

   assert_eq!(UNIHAN_RADICALS.get(&31).unwrap().point, '囗'); 
   assert_eq!(UNIHAN_RADICALS.get(&32).unwrap().point, '土'); 
   assert_eq!(UNIHAN_RADICALS.get(&33).unwrap().point, '士'); 
   assert_eq!(UNIHAN_RADICALS.get(&34).unwrap().point, '夂'); 
   assert_eq!(UNIHAN_RADICALS.get(&35).unwrap().point, '夊'); 
   assert_eq!(UNIHAN_RADICALS.get(&36).unwrap().point, '夕'); 
   assert_eq!(UNIHAN_RADICALS.get(&37).unwrap().point, '大'); 
   assert_eq!(UNIHAN_RADICALS.get(&38).unwrap().point, '女'); 
   assert_eq!(UNIHAN_RADICALS.get(&39).unwrap().point, '子'); 
   assert_eq!(UNIHAN_RADICALS.get(&40).unwrap().point, '宀'); 

   assert_eq!(UNIHAN_RADICALS.get(&41).unwrap().point, '寸'); 
   assert_eq!(UNIHAN_RADICALS.get(&42).unwrap().point, '小'); 
   assert_eq!(UNIHAN_RADICALS.get(&43).unwrap().point, '尢'); 
   assert_eq!(UNIHAN_RADICALS.get(&44).unwrap().point, '尸'); 
   assert_eq!(UNIHAN_RADICALS.get(&45).unwrap().point, '屮'); 
   assert_eq!(UNIHAN_RADICALS.get(&46).unwrap().point, '山'); 
   assert_eq!(UNIHAN_RADICALS.get(&47).unwrap().point, '巛'); 
   assert_eq!(UNIHAN_RADICALS.get(&48).unwrap().point, '工'); 
   assert_eq!(UNIHAN_RADICALS.get(&49).unwrap().point, '己'); 
   assert_eq!(UNIHAN_RADICALS.get(&50).unwrap().point, '巾'); 

   assert_eq!(UNIHAN_RADICALS.get(&51).unwrap().point, '干'); 
   assert_eq!(UNIHAN_RADICALS.get(&52).unwrap().point, '乡'); 
   assert_eq!(UNIHAN_RADICALS.get(&53).unwrap().point, '广'); 
   assert_eq!(UNIHAN_RADICALS.get(&54).unwrap().point, '廴'); 
   assert_eq!(UNIHAN_RADICALS.get(&55).unwrap().point, '廾'); 
   assert_eq!(UNIHAN_RADICALS.get(&56).unwrap().point, '弋'); 
   assert_eq!(UNIHAN_RADICALS.get(&57).unwrap().point, '弓'); 
   assert_eq!(UNIHAN_RADICALS.get(&58).unwrap().point, '彐'); 
   assert_eq!(UNIHAN_RADICALS.get(&59).unwrap().point, '彡'); 
   assert_eq!(UNIHAN_RADICALS.get(&60).unwrap().point, '彳'); 

   assert_eq!(UNIHAN_RADICALS.get(&61).unwrap().point, '心'); 
   assert_eq!(UNIHAN_RADICALS.get(&62).unwrap().point, '戈'); 
   assert_eq!(UNIHAN_RADICALS.get(&63).unwrap().point, '戶'); 
   assert_eq!(UNIHAN_RADICALS.get(&64).unwrap().point, '手'); 
   assert_eq!(UNIHAN_RADICALS.get(&65).unwrap().point, '支'); 
   assert_eq!(UNIHAN_RADICALS.get(&66).unwrap().point, '攴'); 
   assert_eq!(UNIHAN_RADICALS.get(&67).unwrap().point, '文'); 
   assert_eq!(UNIHAN_RADICALS.get(&68).unwrap().point, '斗'); 
   assert_eq!(UNIHAN_RADICALS.get(&69).unwrap().point, '斤'); 
   assert_eq!(UNIHAN_RADICALS.get(&70).unwrap().point, '方'); 

   assert_eq!(UNIHAN_RADICALS.get(&71).unwrap().point, '无'); 
   assert_eq!(UNIHAN_RADICALS.get(&72).unwrap().point, '日'); 
   assert_eq!(UNIHAN_RADICALS.get(&73).unwrap().point, '曰'); 
   assert_eq!(UNIHAN_RADICALS.get(&74).unwrap().point, '月'); 
   assert_eq!(UNIHAN_RADICALS.get(&75).unwrap().point, '木'); 
   assert_eq!(UNIHAN_RADICALS.get(&76).unwrap().point, '欠'); 
   assert_eq!(UNIHAN_RADICALS.get(&77).unwrap().point, '止'); 
   assert_eq!(UNIHAN_RADICALS.get(&78).unwrap().point, '歹'); 
   assert_eq!(UNIHAN_RADICALS.get(&79).unwrap().point, '殳'); 
   assert_eq!(UNIHAN_RADICALS.get(&80).unwrap().point, '毋'); 

   assert_eq!(UNIHAN_RADICALS.get(&81).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&82).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&83).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&84).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&85).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&86).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&87).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&88).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&89).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&90).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&91).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&92).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&93).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&94).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&95).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&96).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&97).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&98).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&99).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&100).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&101).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&102).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&103).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&104).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&105).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&106).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&107).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&108).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&109).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&110).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&111).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&112).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&113).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&114).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&115).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&116).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&117).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&118).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&119).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&120).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&121).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&122).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&123).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&124).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&125).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&126).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&127).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&128).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&129).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&130).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&131).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&132).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&133).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&134).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&135).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&136).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&137).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&138).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&139).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&140).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&141).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&142).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&143).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&144).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&145).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&146).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&147).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&148).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&149).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&150).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&151).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&152).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&153).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&154).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&155).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&156).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&157).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&158).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&159).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&160).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&161).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&162).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&163).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&164).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&165).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&166).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&167).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&168).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&169).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&170).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&171).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&172).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&173).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&174).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&175).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&176).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&177).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&178).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&179).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&180).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&181).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&182).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&183).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&184).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&185).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&186).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&187).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&188).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&189).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&190).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&191).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&192).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&193).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&194).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&195).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&196).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&197).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&198).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&199).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&200).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&201).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&202).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&203).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&204).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&205).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&206).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&207).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&208).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&209).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&210).unwrap().point, ' '); 

   assert_eq!(UNIHAN_RADICALS.get(&211).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&212).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&213).unwrap().point, ' '); 
   assert_eq!(UNIHAN_RADICALS.get(&214).unwrap().point, ' '); 
}
