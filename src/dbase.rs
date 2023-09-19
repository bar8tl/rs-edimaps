//**********************************************************************************
// dbase.rs : Operations over SQLite tables (2019-07-01 bar8tl)
//**********************************************************************************
use crate::settings::SettingsTp;
use crate::utils::{IdxkeyTp, read_index};
use rblib::db::reset_table;
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;

// createdb.rs: SQLite tables creation ---------------------------------------------
const ITABLES: &str = include!("sqlstmts.json");

#[derive(Debug, Clone, Default, Deserialize)]
struct SqlstTp {
  objnm: String,
  activ: String,
  sqlst: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ItablesTp {
  sqlst: Vec<SqlstTp>
}

pub fn crea_tables(s: SettingsTp) {
  let it: ItablesTp = serde_json::from_str(ITABLES).unwrap();
  for sql in &it.sqlst {
    if s.objnm == sql.objnm && sql.activ.to_lowercase() == "yes" {
      reset_table(&s.dbopt, &s.objnm, &sql.sqlst);
      break;
    }
  }
}

// ldrefer.rs: Loads Reference Data Files onto EDIMAPS Database --------------------
#[derive(Debug, Clone, Default, Deserialize)]
struct TranslTp {
  tmedi: String,
  tmode: String,
  tmean: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct TranspTp {
  transp: Vec<TranslTp>
}

#[derive(Debug, Clone, Default, Deserialize)]
struct CodesTp {
  key: String,
  val: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct SapcdTp {
  ctype: String,
  usage: String,
  codes: Vec<CodesTp>
}

#[derive(Debug, Clone, Default, Deserialize)]
struct SapcodesTp {
  sapcodes: Vec<SapcdTp>
}

pub fn load_refdata(s: SettingsTp) {
  let refs = ["indix", "cd_codes", "cd_data"];
  let fncs = [load_index, load_cdcodes, load_cddata];
  fncs[refs.iter().position(|&x| x == s.objnm).unwrap()](s);
}

fn load_index(s: SettingsTp) {
  let cnn  = Connection::open(&s.dbopt).unwrap();
  let indx = read_index(IdxkeyTp{
    mapid: s.objnm.clone(), chgnr: s.sbobj.clone(), idxpt: s.inppt.clone(),
    tabid: s.tabid.clone()}, "ALL");
  cnn.execute("DELETE FROM indix;", ()).expect("Table not reset");
  for c in indx {
    cnn.execute("INSERT INTO indix VALUES
      (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16)",
      (c[0] .clone(), c[1] .clone(), c[2] .clone(), c[3] .clone(), c[4] .clone(),
       c[5] .clone(), c[6] .clone(), c[7] .clone(), c[8] .clone(), c[9] .clone(),
       c[10].clone(), c[11].clone(), c[12].clone(), c[13].clone(), c[14].clone(),
       c[15].clone())
    ).expect("Row not inserted");
  }
  println!("Table 'indix' uploaded.");
}

fn load_cdcodes(s: SettingsTp) {
  let cnn = Connection::open(&s.dbopt).unwrap();
  cnn.execute("DELETE FROM cd_codes;", ()).expect("Table not reset");
  cnn.execute("DELETE FROM cd_index;", ()).expect("Table not reset");
  let f = File::open(s.inppt).expect("Input not found");
  let sapcd: SapcodesTp = from_reader(f).expect("JSON not well-formed");
  for s in sapcd.sapcodes.iter() {
    cnn.execute("INSERT INTO cd_index VALUES (?1,?2)",
      (&s.ctype,&s.usage)).expect("Row not inserted");
    for t in s.codes.iter() {
      cnn.execute("INSERT INTO cd_codes VALUES (?1,?2,?3)",
        (&s.ctype,&t.key,&t.val)).expect("Row not inserted");
    }
  }
  println!("Table 'cd_index' uploaded.");
  println!("Table 'cd_codes' uploaded.");
}

fn load_cddata(s: SettingsTp) {
  let cnn = Connection::open(&s.dbopt).unwrap();
  cnn.execute("DELETE FROM cd_data;", ()).expect("Table not reset");
  let f = File::open(s.inppt).expect("Input not found");
  let trnsp: TranspTp = from_reader(f).expect("JSON not well-formed");
  for t in trnsp.transp.iter() {
    cnn.execute("INSERT INTO cd_data VALUES (?1,?2,?3,?4)",
     ("editransp",&t.tmedi,&t.tmode,&t.tmean)).expect("Row not inserted");
  }
  println!("Table 'cd_data' uploaded.");
}

//----------------------------------------------------------------------------------
pub fn dspl_mapspecs(s: SettingsTp) {
  let cnn  = Connection::open(&s.dbopt).unwrap();
  let indx = read_index(IdxkeyTp{
    mapid: s.mapid.clone(), chgnr: s.chgnr.clone(), idxpt: s.idxpt.clone(),
    tabid: s.tabid.clone()}, "ALL");
  for c in indx {
    if c[14] == s.objnm {
      if s.sbobj == "count" {
        let mut count: usize = 0;
        cnn.query_row("SELECT count(*) FROM mapspecs WHERE mapid=?1 and chgnr=?2;",
          [c[0].to_string(), c[10].to_string()], |row| {
            Ok(count = row.get(0).unwrap()) })
          .expect("Error: Segment type not found in definition DB");
        println!("{},{},{}", c[0], c[10], count);
      } else {
        let mut stmt = cnn.prepare(
          "SELECT mapid,chgnr,grpid,sgmid,targt,rowno,seqno from mapspecs
            WHERE mapid=?1 and chgnr=?2;").unwrap();
        let mut rows = stmt.query([c[0].to_string(), c[10].to_string(),]).unwrap();
        while let Some(row) = rows.next().expect("while row failed") {
          let mapid: String = row.get(0).unwrap();
          let chgnr: String = row.get(1).unwrap();
          let grpid: String = row.get(2).unwrap();
          let sgmid: String = row.get(3).unwrap();
          let targt: String = row.get(4).unwrap();
          let rowno: String = row.get(5).unwrap();
          let seqno: String = row.get(6).unwrap();
          println!("{},{},{},{},{},{},{}", mapid, chgnr, grpid, sgmid, targt,
            rowno, seqno);
        }
      }
    }
  }
}
