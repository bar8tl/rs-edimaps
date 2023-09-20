//**********************************************************************************
// mapspecs.rs: Starts archive processes for EDI messages mapping specifications
// (2019-07-01 bar8tl)
//**********************************************************************************
#![allow(unused_assignments)]

use crate::settings::SettingsTp;
use crate::utils::fmt_outvalues;
use calamine::{Reader, Xlsx, open_workbook, RangeDeserializerBuilder, Error};
use chrono::{Datelike, Duration, NaiveDate};
use rusqlite::Connection;

pub fn deser_mapspec(s: SettingsTp) {
  let mtyp = ["crl", "inv", "asn"];
  let fncs = [proc_mapcrl, proc_mapinv, proc_mapasn];
  fncs[mtyp.iter().position(|&x| x == s.msgtp).unwrap()](s);
}

// cr=EDI Customer Releases (830,850,860,862,DELFOR,SEQJIT). New and changes -------
type CrlinTp = (String, String, String, String, String, String, String);
type CrrowTp = [String; 7];

#[derive(Debug, Clone, Default)]
struct CrhdrTp {
  mptit: String,
  lstup: String,
  authr: String,
  bvers: String,
  custm: String,
  tform: String,
  sform: String
}

#[derive(Debug, Clone, Default)]
struct CrTp {
  hdr  : CrhdrTp,
  strdt: NaiveDate,
  mapid: String,
  chgnr: String,
  rowno: String,
  inhdr: String,
  ingrp: String,
  insgm: String,
  trims: String,
  lfchr: String,
  sqhdr: i16,
  sqgrp: i16,
  sqsgm: i16,
  sqfld: i16,
  nextr: bool,
  frgrp: bool,
  endcl: bool
}

fn proc_mapcrl(s: SettingsTp) {
  let mut cr = CrTp { ..Default::default() };
  let mut cl : CrrowTp = Default::default();
  let cnn = Connection::open(&s.dbopt).unwrap();
  init_crdata(&s, &cnn, &mut cr);
  let mut workbook: Xlsx<_> = open_workbook(format!("{}{}\\{}", s.mapdr, s.ctmrl,
    s.fname)).expect("Input not found");
  let range = workbook.worksheet_range("Mapping")
    .ok_or(Error::Msg("Cannot find specified tab")).unwrap().unwrap();
  let iter = RangeDeserializerBuilder::new().has_headers(false).from_range(&range)
    .unwrap();
  for (j, i) in iter.enumerate() {
    let l: CrlinTp = i.expect("Row not mapped");
    cl = fmt_outvalues([l.0,l.1,l.2,l.3,l.4,l.5,l.6], &cr.trims, &cr.lfchr);
    cr.rowno = format!("{:04}", j);
    proc_linebyline(&cnn, &mut cr, &cl);
  }
  println!("Records |{:4}|{:4}|{:4}|{:4}|", cr.sqhdr, cr.sqgrp, cr.sqsgm, cr.sqfld);
}

fn proc_linebyline(cnn: &Connection, cr: &mut CrTp, cl: &[String; 7]) {
  // Header lines
  if cr.inhdr.len() == 0 {
    if cl[1].to_lowercase().contains("common mapping") {
      prep_crhdr(cl, cr);
      cr.frgrp = false;
      return();
    }
  }
  if cr.ingrp == "HDR" {
    if !cr.endcl {
      isrt_crhdr(cnn, cl, cr);

  // Control record lines
    } else if cl[2].to_lowercase().contains("control record"   ) ||
              cl[2].to_lowercase().contains("edi segment/field") {
      cr.ingrp = "CTRL".to_string();
      isrt_cregrp(cnn, cr);
      isrt_crsgms(cnn, cl, cr);
    }

  // First section or first segment lines after Header or Control record
  } else if cr.ingrp == "CTRL" {
    if cl[2].to_lowercase().contains("section") ||
       cl[2].to_lowercase().contains("group"  ) {
      cr.frgrp = true;
      isrt_crgrps(cnn, cl, cr);
    } else
    if cl[2].to_lowercase().contains("segment") {
      if !cr.frgrp {
        cr.ingrp = "MAIN".to_string();
        isrt_cregrp(cnn, cr);
      }
      isrt_crsgms(cnn, cl, cr);
    } else {
      isrt_crflds(cnn, cl, cr);
      cr.frgrp = false;
    }

  // Subsequent section and segment lines
  } else {
    if cl[2].to_lowercase().contains("section") ||
       cl[2].to_lowercase().contains("group"  ) {
      isrt_crgrps(cnn, cl, cr);
    } else
    if cl[2].to_lowercase().contains("segment:") {
      isrt_crsgms(cnn, cl, cr);
    } else {
      isrt_crflds(cnn, cl, cr);
    }
  }
}

fn init_crdata(s: &SettingsTp, cnn: &Connection, cr: &mut CrTp) {
  cr.mapid = s.mapid.clone();
  cr.chgnr = s.chgnr.clone();
  cr.trims = s.trims.clone();
  cr.lfchr = s.lfchr.clone();
  cr.endcl = false;
  cr.strdt = NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in start date");
  cnn.execute("DELETE FROM mapspecs where mapid=?1 and chgnr=?2;",
   (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM headers  where mapid=?1 and chgnr=?2;",
   (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM groups   where mapid=?1 and chgnr=?2;",
   (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM segments where mapid=?1 and chgnr=?2;",
   (&cr.mapid, &cr.chgnr)).expect("Table not reset");
  cnn.execute("DELETE FROM fields   where mapid=?1 and chgnr=?2;",
   (&cr.mapid, &cr.chgnr)).expect("Table not reset");
}

fn prep_crhdr(cl: &[String; 7], cr: &mut CrTp) {
  cr.inhdr = cr.mapid.clone();
  cr.ingrp = "HDR".to_string();
  cr.mapid = cr.inhdr.clone();
  cr.hdr.mptit = cl[1].clone();
}

fn isrt_crhdr(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp) {
  if cl[3].contains("Author") {
    cr.hdr.lstup = cl[2].clone();
    cr.hdr.authr = cl[4].clone();
  } else if cl[3].contains("Customer") {
    cr.hdr.bvers = cl[2].clone();
    cr.hdr.custm = cl[4].clone();
  } else if cl[0].contains("Field") {
    cr.nextr = true;
  } else if cl[2].len() > 0 && cr.nextr {
    cr.hdr.tform = cl[2].clone();
    cr.hdr.sform = cl[3].clone();
    cr.nextr = false;
    cr.sqhdr += 1;
    let seqno = format!("{:04}", cr.sqhdr);
    let lupdt = cr.hdr.lstup.parse::<i64>().unwrap();
    let hdrdt = cr.strdt.checked_add_signed(Duration::days(lupdt-2)).unwrap();
    let lstup = format!("{}-{:02}-{:02}", hdrdt.year(), hdrdt.month(), hdrdt.day());
    cnn.execute("INSERT INTO headers VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
     (&cr.mapid, &cr.chgnr, &cr.hdr.mptit, lstup, &cr.hdr.authr, &cr.hdr.bvers,
      &cr.hdr.custm, &cr.hdr.tform, &cr.hdr.sform, &cr.rowno, &seqno))
     .expect("Header row not inserted");
    cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
     (&cr.mapid, &cr.chgnr, &"".to_string(), &"".to_string(), &"".to_string(),
      &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
    if cr.chgnr.len() > 0 {
      cr.ingrp = "CTRL".to_string();
    }
    cr.endcl = true;
  }
}

fn isrt_cregrp(cnn: &Connection, cr: &mut CrTp) {
  cr.sqgrp += 1;
  let seqno = format!("{:04}", cr.sqgrp);
  cnn.execute("INSERT INTO groups VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
   (&cr.mapid, &cr.chgnr, &cr.ingrp, &"".to_string(), &"".to_string(),
    &"".to_string(), &"".to_string(), &"".to_string(), &cr.rowno, &seqno))
   .expect("Section row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
   (&cr.mapid, &cr.chgnr, &cr.ingrp, &"".to_string(), &"".to_string(),
    &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

fn isrt_crgrps(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp) {
  cr.sqgrp += 1;
  let seqno = format!("{:04}", cr.sqgrp);
  cr.ingrp = "MAIN".to_string();
  if let Some(idx) = cl[2].find(": ") {
    cr.ingrp = cl[2][idx+2..cl[2].len()].to_string();
  }
  cnn.execute("INSERT INTO groups VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)",
   (&cr.mapid, &cr.chgnr, &cr.ingrp, &cl[3], &cl[4], &cl[5], &cl[0], &cl[1],
    &cr.rowno, &seqno))
   .expect("Section row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
   (&cr.mapid, &cr.chgnr, &cr.ingrp, &"".to_string(), &"".to_string(),
    &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

fn isrt_crsgms(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp) {
  cr.sqsgm += 1;
  let seqno = format!("{:04}", cr.sqsgm);
  cr.insgm = String::new();
  if let Some(idx) = cl[2].find(": ") {
    cr.insgm = cl[2][idx+2..cl[2].len()].to_string();
  }
  let mut sgmtp = String::new();
  if let Some(idx) = cl[3].find(": ") {
    sgmtp = cl[3][idx+2..cl[3].len()].to_string();
  }
  cnn.execute("INSERT INTO segments VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
   (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &sgmtp, &cl[4], &cl[5], &cl[0],
    &cl[1], &cr.rowno, &seqno))
   .expect("Segment row not inserted");
  cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
   (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &"".to_string(),
    &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
}

fn isrt_crflds(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp) {
  if cl[0].len() > 0 || cl[1].len() > 0 || cl[2].len() > 0 || cl[3].len() > 0 ||
     cl[4].len() > 0 || cl[5].len() > 0 || cl[6].len() > 0 {
    cr.sqfld += 1;
    let seqno = format!("{:04}", cr.sqfld);
    cnn.execute(
    "INSERT INTO fields VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
     (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &cl[2], &cl[3], &cl[4], &cl[5],
      &cl[0], &cl[1], &cr.rowno, &seqno, &cl[6]))
     .expect("Field row not inserted");
    cnn.execute("INSERT INTO mapspecs VALUES (?1,?2,?3,?4,?5,?6,?7)",
     (&cr.mapid, &cr.chgnr, &cr.ingrp, &cr.insgm, &cl[2],
      &cr.rowno, &seqno)).expect("Mapspecs row not inserted");
  }
}

// in=EDI Invoices (810,INVOICE). New and changes ----------------------------------
fn proc_mapinv(s: SettingsTp) {
  if s.templ == "outcm" {
    proc_mapcrl(s);
  }
}

// as=EDI ASNs (856,DESADV). New and changes ---------------------------------------
fn proc_mapasn(_s: SettingsTp) {}
