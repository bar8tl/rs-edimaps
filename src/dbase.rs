//**********************************************************************************
// dbase.rs : Operations over SQLite tables (2017-05-24 bar8tl)
//**********************************************************************************
use crate::settings::SettingsTp;
use calamine::{Reader, Xlsx, open_workbook, RangeDeserializerBuilder, Error};
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
  let cnn = Connection::open(&s.dbopt).unwrap();
  let mut workbook: Xlsx<_> = open_workbook(s.inppt).expect("Input not found");
  let range = workbook.worksheet_range(s.tabid.as_str())
    .ok_or(Error::Msg("Cannot find specified tab")).unwrap().unwrap();
  let iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();
  cnn.execute("DELETE FROM indix;", ()).expect("Table not reset");
  for i in iter {
    let msgtp = String::new();
    let (mapid,  ctmrs,  ctmrl,  messg,  mvers,  idocm,  idoct,  mstat,  fname,
         relsd,  chgnr,  suprt,  asgnd,  dstat):
        (String, String, String, String, String, String, String, String, String,
         String, String, String, String, String) = i.expect("Row not mapped");
    cnn.execute("INSERT INTO indix VALUES
      (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
      (mapid, ctmrs, ctmrl, messg, mvers, idocm, idoct, mstat, fname,
       relsd, chgnr, suprt, asgnd, dstat, msgtp)
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
