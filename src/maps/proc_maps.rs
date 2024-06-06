// proc_maps.rs - Function modules to process EDI Mapping specification to add it to
// the repository or to generate a json formated output file (2021-07-01 bar8tl)
use crate::assets::{IdxkeyTp, read_index};
use crate::config::{RefersTp, MapsTp};
use crate::maps::tojson::{init_cr_json, isrt_crhdr_json, isrt_cregrp_json,
  isrt_crgrps_json, isrt_crsgms_json, isrt_crflds_json, write_cr_json, SpecsTp};
use crate::maps::torepo::{init_cr_repo, isrt_crhdr_repo, isrt_cregrp_repo,
  isrt_crgrps_repo, isrt_crsgms_repo, isrt_crflds_repo};
use calamine::{Reader, Xlsx, open_workbook, RangeDeserializerBuilder, Error};
use chrono::{NaiveDate, Datelike, Duration};
use rusqlite::Connection;

// types.rs - Data types required for processing mapping specification
// (2021-07-01 bar8tl)
pub type CrlinTp = (String, String, String, String, String, String, String);
pub type CrrowTp = [String; 7];

#[derive(Debug, Clone, Default)]
pub struct CrhdrTp {
  pub mptit: String,
  pub lstup: String,
  pub authr: String,
  pub bvers: String,
  pub custm: String,
  pub tform: String,
  pub sform: String
}

#[derive(Debug, Clone, Default)]
pub struct CrTp {
  pub hdr  : CrhdrTp,
  pub strdt: NaiveDate,
  pub mapid: String,
  pub chgnr: String,
  pub rowno: String,
  pub inhdr: String,
  pub ingrp: String,
  pub insgm: String,
  pub trims: String,
  pub lfchr: String,
  pub templ: String,
  pub sqhdr: i16,
  pub sqgrp: i16,
  pub sqsgm: i16,
  pub sqfld: i16,
  pub ixgrp: i16,
  pub ixsgm: i16,
  pub nextr: bool,
  pub frgrp: bool,
  pub endcl: bool
}

// proc_maps.rs - Starts processes for EDI messages mapping specifications
// (2021-07-01 bar8tl)
// Command line: edimaps map -r -j <mapping-specs-id>
pub fn proc_maps(dbopt: &String, rfr: &RefersTp, map: &MapsTp, repo: bool,
  json: bool) {
  let d = get_mapdetail(rfr, map);
  let mtyp = ["crl", "inv", "asn"];
  let fncs = [proc_mapcrl, proc_mapinv, proc_mapasn];
  fncs[mtyp.iter().position(|&x| x == d.msgtp).unwrap()] (dbopt, map, &d, repo, json);
}

// proc_mapcrl.rs - Process CR (Customer Release) mapping specs (2021-07-01 bar8tl)
fn proc_mapcrl(dbopt: &String, map: &MapsTp, d: &IdxdatTp, repo: bool, json: bool) {
  let mut cr = CrTp    { ..Default::default() };
  let mut sp = SpecsTp { ..Default::default() };
  let cnn = Connection::open(dbopt).unwrap();
  init_crdata(&d.mapid, &d.chgnr, map.trims.clone(), map.lfchr.clone(), &d.templ,
    &cnn, &mut cr, repo, json, &mut sp);
  let mut workbook: Xlsx<_> = open_workbook(format!("{}{}\\{}", map.mapdr, d.ctmrl,
    d.fname)).expect("Input not found");
  let range = workbook.worksheet_range("Mapping")
    .ok_or(Error::Msg("Cannot find specified tab")).unwrap().unwrap();
  let iter = RangeDeserializerBuilder::new().has_headers(false).from_range(&range)
    .unwrap();
  for (j, i) in iter.enumerate() {
    let  l: CrlinTp = i.expect("Row not mapped");
    let cl: CrrowTp = fmt_columns([l.0,l.1,l.2,l.3,l.4,l.5,l.6], &cr.trims,
      &cr.lfchr);
    cr.rowno = format!("{:04}", j);
    proc_linebyline(&cnn, &mut cr, &cl, repo, json, &mut sp);
  }
  if json { write_cr_json(&map.bkpdr, &d, &sp); }
  println!("Records |{:4}|{:4}|{:4}|{:4}|", cr.sqhdr, cr.sqgrp, cr.sqsgm, cr.sqfld);
}

fn init_crdata(mapid: &String, chgnr: &String, trims: String, lfchr: String,
  templ: &String, cnn: &Connection, cr: &mut CrTp, repo: bool, json: bool,
  sp: &mut SpecsTp) {
  cr.mapid = mapid.clone();
  cr.chgnr = chgnr.clone();
  cr.trims = trims.clone();
  cr.lfchr = lfchr.clone();
  cr.templ = templ.clone();
  cr.endcl = false;
  cr.strdt = NaiveDate::from_ymd_opt(1900, 1, 1).expect("Error in start date");
  if repo { init_cr_repo(cnn, cr); }
  if json { init_cr_json(sp); }
}

// proc_linebyline.rs - Process CR mapping specs in MS Excel file line by line
// (2021-07-01 bar8tl)
fn proc_linebyline(cnn: &Connection, cr: &mut CrTp, cl: &[String; 7], repo: bool,
  json: bool, sp: &mut SpecsTp) {
  // Header lines
  if cr.inhdr.len() == 0 {
    if cl[1].to_lowercase().contains("common mapping") {
      cr.hdr.mptit = cl[1].clone();
      cr.inhdr = cr.mapid.clone();
      cr.ingrp = "HDR".to_string();
      cr.frgrp = false;
      return();
    }
  }
  if cr.ingrp == "HDR" {
    if !cr.endcl {
      isrt_crhdr(cnn, cl, cr, repo, json, sp);

  // Control record lines
    } else if cl[2].to_lowercase().contains("control record"   ) ||
              cl[2].to_lowercase().contains("edi segment/field") {
      cr.ingrp = "CTRL".to_string();
      isrt_cregrp(cnn, cr, repo, json, sp);
      isrt_crsgms(cnn, cl, cr, repo, json, sp);

  // First section or first segment lines after Header (mapsp without Control record)
    } else {
      if (cl[2].to_lowercase().contains("section") &&
         !cl[2].to_lowercase().starts_with("segment")) ||
          cl[2].to_lowercase().starts_with("group"  )  {
        cr.frgrp = true;
        isrt_crgrps(cnn, cl, cr, repo, json, sp);
      } else
      if cl[2].to_lowercase().starts_with("segment") {
        if !cr.frgrp {
          cr.ingrp = "MAIN".to_string();
          isrt_cregrp(cnn, cr, repo, json, sp);
        }
        isrt_crsgms(cnn, cl, cr, repo, json, sp);
      } else {
        isrt_crflds(cnn, cl, cr, repo, json, sp);
        cr.frgrp = false;
      }
    }

  // First section or first segment lines after Control record
  } else if cr.ingrp == "CTRL" {
    if (cl[2].to_lowercase().contains("section") &&
       !cl[2].to_lowercase().starts_with("segment")) ||
        cl[2].to_lowercase().starts_with("group"  )  {
      cr.frgrp = true;
      isrt_crgrps(cnn, cl, cr, repo, json, sp);
    } else
    if cl[2].to_lowercase().starts_with("segment") {
      if !cr.frgrp {
        cr.ingrp = "MAIN".to_string();
        isrt_cregrp(cnn, cr, repo, json, sp);
      }
      isrt_crsgms(cnn, cl, cr, repo, json, sp);
    } else {
      isrt_crflds(cnn, cl, cr, repo, json, sp);
      cr.frgrp = false;
    }

  // Subsequent section and segment lines
  } else {
    if (cl[2].to_lowercase().contains("section") &&
       !cl[2].to_lowercase().starts_with("segment")) ||
        cl[2].to_lowercase().starts_with("group"  )  {
      isrt_crgrps(cnn, cl, cr, repo, json, sp);
    } else
    if cl[2].to_lowercase().starts_with("segment:") {
      isrt_crsgms(cnn, cl, cr, repo, json, sp);
    } else
    if !cl[5].to_lowercase().contains("end of mapping") {
      isrt_crflds(cnn, cl, cr, repo, json, sp);
    }
  }
}

// isrt_crhdr.rs - Insert header records (2021-07-01 bar8tl)
fn isrt_crhdr(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp, repo: bool,
  json: bool, sp: &mut SpecsTp) {
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
    cr.ixgrp  = 0;
    cr.ixsgm  = 0;
    let seqno = format!("{:04}", cr.sqhdr);
    let lupdt = cr.hdr.lstup.parse::<i64>().unwrap();
    let hdrdt = cr.strdt.checked_add_signed(Duration::days(lupdt-2)).unwrap();
    let lstup = format!("{}-{:02}-{:02}", hdrdt.year(), hdrdt.month(), hdrdt.day());
    if repo { isrt_crhdr_repo(cnn, cr, &lstup, &seqno); }
    if json { isrt_crhdr_json(cr, &lstup, sp); }
    //if cr.chgnr.len() > 0 {
    //  cr.ingrp = "CTRL".to_string();
    //}
    cr.endcl = true;
  }
}

// isrt_cregrp.rs - Insert first group record (2021-07-01 bar8tl)
fn isrt_cregrp(cnn: &Connection, cr: &mut CrTp, repo: bool, json: bool,
  sp: &mut SpecsTp) {
  cr.sqgrp += 1;
  cr.ixgrp += 1;
  cr.ixsgm  = 0;
  let seqno = format!("{:04}", cr.sqgrp);
  if repo { isrt_cregrp_repo(cnn, cr, &seqno); }
  if json { isrt_cregrp_json(cr, sp);          }
}

// isrt_crgrps.rs - Insert subsequent group records (2021-07-01 bar8tl)
fn isrt_crgrps(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp, repo: bool,
  json: bool, sp: &mut SpecsTp) {
  cr.sqgrp += 1;
  cr.ixgrp += 1;
  cr.ixsgm  = 0;
  let seqno = format!("{:04}", cr.sqgrp);
  cr.ingrp = "MAIN".to_string();
  if let Some(idx) = cl[2].find(": ") {
    cr.ingrp = cl[2][idx+2..cl[2].len()].to_string();
  }
  if repo { isrt_crgrps_repo(cnn, cl, cr, &seqno); }
  if json { isrt_crgrps_json(cl, cr, sp);          }
}

// isrt_crtsgms.rs - Insert segment records (2021-07-01 bar8tl)
fn isrt_crsgms(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp, repo: bool,
  json: bool, sp: &mut SpecsTp) {
  cr.sqsgm += 1;
  cr.ixsgm += 1;
  let seqno = format!("{:04}", cr.sqsgm);
  cr.insgm = String::new();
  if let Some(idx) = cl[2].find(": ") {
    cr.insgm = cl[2][idx+2..cl[2].len()].to_string();
  }
  let mut sgmtp = String::new();
  if let Some(idx) = cl[3].find(": ") {
    sgmtp = cl[3][idx+2..cl[3].len()].to_string();
  }
  if repo { isrt_crsgms_repo(cnn, cl, cr, &sgmtp, &seqno); }
  if json { isrt_crsgms_json(cl, cr, &sgmtp, sp);          }
}

// isrt_crflds.rs - Insert field records (2021-07-01 bar8tl)
fn isrt_crflds(cnn: &Connection, cl: &[String; 7], cr: &mut CrTp, repo: bool,
  json: bool, sp: &mut SpecsTp) {
  if cl[0].len() > 0 || cl[1].len() > 0 || cl[2].len() > 0 || cl[3].len() > 0 ||
     cl[4].len() > 0 || cl[5].len() > 0 || cl[6].len() > 0 {
    cr.sqfld += 1;
    let seqno = format!("{:04}", cr.sqfld);
    if repo { isrt_crflds_repo(cnn, cl, cr, &seqno); }
    if json { isrt_crflds_json(cl, cr, sp);          }
  }
}

// proc_mapinv.rs - Process INVOICE mapping specs (2021-07-01 bar8tl)
// in=EDI Invoices (810,INVOICE). New and changes
fn proc_mapinv(dbopt: &String, map: &MapsTp, d: &IdxdatTp, repo: bool, json: bool) {
  if d.templ == "outcm" { // specs using rbna common template can use crl procedure
    proc_mapcrl(dbopt, map, d, repo, json);
  }
}

// proc_mapasn.rs - Process ASN mapping specs (2021-07-01 bar8tl)
// as=EDI ASNs (856,DESADV). New and changes
fn proc_mapasn(dbopt: &String, map: &MapsTp, d: &IdxdatTp, repo: bool, json: bool) {
  // pending to develop (consider different specs format templates being used)
  println!("|{}|{:?}|{:?}|{}|{}|", dbopt, map, d, repo, json);
}

// get_mapdetail.rs - Get EDI mapping specs detail into an arrangement from internal
// table previuosly obtained from an excel list (2021-07-01 bar8tl)
#[derive(Debug, Clone, Default)]
pub struct IdxdatTp {
  pub mapid: String,
  pub ctmrs: String,
  pub ctmrl: String,
  pub messg: String,
  pub mvers: String,
  pub idocm: String,
  pub mstat: String,
  pub fname: String,
  pub relsd: String,
  pub chgnr: String,
  pub suprt: String,
  pub asgnd: String,
  pub dstat: String,
  pub templ: String,
  pub msgtp: String
}

fn get_mapdetail(rfr: &RefersTp, map: &MapsTp) -> IdxdatTp {
  let mut d = IdxdatTp { ..Default::default() };
  let indx = read_index(IdxkeyTp{
    mapid: map.mapid.clone(), chgnr: map.chgnr.clone(), idxpt: rfr.idxpt.clone(),
    tabid: rfr.tabid.clone()}, "SINGLE");
  (d.mapid, d.ctmrs, d.ctmrl, d.messg, d.mvers, d.idocm, d.idocm, d.mstat,
   d.fname, d.relsd, d.chgnr, d.suprt, d.asgnd, d.dstat, d.templ, d.msgtp) =
  (indx[0][0] .clone(), indx[0][1] .clone(), indx[0][2] .clone(),
   indx[0][3] .clone(), indx[0][4] .clone(), indx[0][5] .clone(),
   indx[0][6] .clone(), indx[0][7] .clone(), indx[0][8] .clone(),
   indx[0][9] .clone(), indx[0][10].clone(), indx[0][11].clone(),
   indx[0][12].clone(), indx[0][13].clone(), indx[0][14].clone(),
   indx[0][15].clone());
   return d;
}

// fmt_columns.rs - Format mapping fields of all columns (2021-07-01 bar8tl)
fn fmt_columns(cl: [String; 7], trims: &String, lfchr: &String) -> [String; 7] {
  let mut c: [String; 7] = Default::default();
  for i in 0..cl.len() {
    c[i] = cl[i].replace("\r\n", lfchr);
    if trims == "yes" {
      c[i] = c[i].trim().to_string();
    }
  }
  return c;
}
