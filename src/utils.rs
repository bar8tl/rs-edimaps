//**********************************************************************************
// utils.rs: Utility functions (2019-07-01 BAR8TL)
//**********************************************************************************
use calamine::{Reader, Xlsx, open_workbook, RangeDeserializerBuilder, Error};

type IdxlinTp = (String, String, String, String, String, String, String, String,
                 String, String, String, String, String, String, String);
type IdxrowTp = [String; 16];

//----------------------------------------------------------------------------------
pub struct IdxkeyTp {
  pub idxpt: String,
  pub tabid: String,
  pub mapid: String,
  pub chgnr: String
}

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
      if messg == "invoice" || messg == "810" { "inv".to_string() } else {
      if messg == "desadv"  || messg == "856" { "asn".to_string() } else {
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

//----------------------------------------------------------------------------------
pub fn fmt_outvalues(cl: [String; 7], trims: &String, lfchr: &String) ->
  [String; 7] {
  let mut c: [String; 7] = Default::default();
  for i in 0..cl.len() {
    c[i] = cl[i].replace("\r\n", lfchr);
    if trims == "yes" {
      c[i] = c[i].trim().to_string();
    }
  }
  return c;
}
