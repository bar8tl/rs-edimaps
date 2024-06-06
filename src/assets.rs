// assets.rs - Function modules to upload reference information to the edimaps
// program repository (2021-07-01 bar8tl)
use calamine::{Reader, Xlsx, open_workbook, RangeDeserializerBuilder, Error};
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;

// add_cdcodes.rs - Add SAP EDI IDocs codes to the repository (2021-07-01 bar8tl)
// Command line: emi add -r cdcodes
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

pub fn add_cdcodes(dbpath: &String, rfpath: String) {
  let cnn = Connection::open(dbpath).expect("Error opening DB");
  cnn.execute("DELETE FROM cdcodes;", ()).expect("Table not reset");
  cnn.execute("DELETE FROM cdindex;", ()).expect("Table not reset");
  let f = File::open(rfpath).expect("Input not found");
  let sapcd: SapcodesTp = from_reader(f).expect("JSON not well-formed");
  for sc in sapcd.sapcodes.iter() {
    cnn.execute("INSERT INTO cdindex VALUES (?1,?2);", (&sc.ctype, &sc.usage))
      .expect("Row not inserted");
    for st in sc.codes.iter() {
      cnn.execute("INSERT INTO cdcodes VALUES (?1,?2,?3);", (&sc.ctype, &st.key,
        &st.val)).expect("Row not inserted");
    }
  }
  println!("Table 'cdindex' uploaded.");
  println!("Table 'cdcodes' uploaded.");
}

// add_cddata.rs - Add specific EDI-SAP Idoc code equivalences to the repository.
// Example: Standard EDI Transport means codes (2021-07-01 bar8tl)
// Command line: edimaps add -r cddata
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

pub fn add_cddata(dbpath: &String, rfpath: String) {
  let cnn = Connection::open(dbpath).expect("DB Open Error");
  cnn.execute("DELETE FROM cddata;", ()).expect("Table not reset");
  let f = File::open(rfpath).expect("Input not found");
  let trnsp: TranspTp = from_reader(f).expect("JSON not well-formed");
  for st in trnsp.transp.iter() {
    cnn.execute("INSERT INTO cddata VALUES (?1,?2,?3,?4)",
     ("editransp",&st.tmedi,&st.tmode,&st.tmean)).expect("Row not inserted");
  }
  println!("Table 'cddata' uploaded.");
}

// add_idoctp.rs - Add IDOC type additional data to allow idntification of raw files
// from SAP systems (2021-07-01 bar8tl)
// Command line: edimaps add -r idoctp
#[derive(Debug, Clone, Default, Deserialize)]
pub struct IdoctpTp {
  pub itype: String,
  #[serde(default)]
  pub idefn: String,
  pub short: String,
  #[serde(default)]
  pub cntrl: String,
  #[serde(default)]
  pub clien: String,
  #[serde(default)]
  pub rcvpf: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct IdoctplTp {
  idoct: Vec<IdoctpTp>
}

pub fn add_idoctp(dbpath: &String, rfpath: String) {
  let cnn = Connection::open(dbpath).expect("DB Open Error");
  cnn.execute("DELETE FROM idoctp;", ()).expect("Table not reset");
  let f = File::open(rfpath).expect("Input not found");
  let idtpl: IdoctplTp = from_reader(f).expect("JSON not well-formed");
  for it in idtpl.idoct.iter() {
    cnn.execute("INSERT INTO idoctp VALUES (?1,?2,?3,?4,?5,?6)",
     (&it.itype, String::new(), &it.short, String::new(), String::new(), &it.rcvpf))
     .expect("Row not inserted");
  }
  println!("Table 'idoctp' uploaded.");
}

// add_wkflow.rs - Add Idoc process workflow configuration file (2021-07-01 bar8tl)
// Command line: edimaps add -r wkflow
#[derive(Debug, Clone, Default, Deserialize)]
pub struct StepTp {
  pub step : String,
  pub inpdr: String,
  pub inptp: String,
  pub outdr: String,
  pub outtp: String,
  #[serde(default)]
  pub refdr: String,
  #[serde(default)]
  pub reftp: String,
  pub wkflw: String,
  pub pcddr: String,
  pub ifilt: String
}

#[derive(Debug, Clone, Default, Deserialize)]
struct StepsTp {
  steps: Vec<StepTp>
}

pub fn add_wkflow(dbpath: &String, rfpath: String) {
  let cnn = Connection::open(dbpath).expect("DB Open Error");
  cnn.execute("DELETE FROM wkflow;", ()).expect("Table not reset");
  let f = File::open(rfpath).expect("Input not found");
  let steps: StepsTp = from_reader(f).expect("JSON not well-formed");
  for st in steps.steps.iter() {
    cnn.execute("INSERT INTO wkflow VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
     (&st.step,  &st.inpdr, &st.inptp, &st.outdr, &st.outtp, &st.refdr, &st.reftp,
      &st.wkflw, &st.pcddr, &st.ifilt)).expect("Row not inserted");
  }
  println!("Table 'wkflow' uploaded.");
}

// index.rs - Function modules to upload the mapping specification index file to
// the emi progra repository (2021-07-01 bar8tl)
// Command line: edimaps add -r index
type IdxlinTp = (String, String, String, String, String, String, String,
 String, String, String, String, String, String, String, String);
type IdxrowTp = [String; 16];

#[derive(Debug, Clone, Default)]
pub struct IdxkeyTp {
  pub idxpt: String,
  pub tabid: String,
  pub mapid: String,
  pub chgnr: String
}

// add_index.rs - Add index of EDI mapping specification files to the repository
// (from an external MS-Excel file (2021-07-01 bar8tl)
pub fn add_index(dbpath: &String, rfpath: &String, tabid: &String) {
  let cnn  = Connection::open(dbpath).expect("DB Open Error");
  cnn.execute("DELETE FROM indix;", ()).expect("Table not reset");
  let indx = read_index(IdxkeyTp{
    idxpt: rfpath.clone(), tabid: tabid.clone(), mapid: "".to_string(),
    chgnr: "".to_string()}, "ALL");
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

// read_index.rs - Retrieves specific detail about a selected mapping specification
// file from the index MS Excel file (2021-07-01 bar8tl)
pub fn read_index(p: IdxkeyTp, mode: &str) -> Vec<IdxrowTp> {
  let mut cell: Vec<IdxrowTp> = vec![];
  let mut cl  : IdxrowTp;
  let mut workbook: Xlsx<_> = open_workbook(p.idxpt).expect("Input not found");
  let range = workbook.worksheet_range(p.tabid.as_str())
    .ok_or(Error::Msg("Cannot find specified tab")).unwrap().unwrap();
  let iter = RangeDeserializerBuilder::new().from_range(&range).unwrap();
  for i in iter {
    let (mapid, ctmrs, ctmrl, messg, mvers, idocm, idoct, mstat, fname, relsd,
         chgnr, suprt, asgnd, dstat, templ): IdxlinTp = i.expect("Row not mapped");
    cl = [mapid.clone(), ctmrs, ctmrl, messg.clone(), mvers, idocm, idoct, mstat,
          fname, relsd, chgnr.clone(), suprt, asgnd, dstat, templ, String::new()];
    cl[15] =
      if messg == "invoic" || messg == "810" { "inv".to_string() } else {
      if messg == "desadv" || messg == "856" { "asn".to_string() } else {
                                               "crl".to_string() }};
    if mode == "SINGLE" {
      if mapid == p.mapid && chgnr == p.chgnr {
        cell.push(cl.clone());
        break;
      }
    } else {
      cell.push(cl.clone());
    }
  }
  return cell;
}
