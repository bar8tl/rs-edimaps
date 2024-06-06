// tojson.rs - Starts proper function to convert IDOC content from fixed size
// format to json format. Either from a set of files contained within a folder or
// from an specific single file (2021-07-01 bar8tl)
use crate::assets::IdoctpTp;
use crate::definitn::{OutitmTp, OutstrTp};
use crate::readidoc::read_idocs::next_stage;
use crate::readidoc::read_idocs::{StageTp, get_idoctp};
use rblib::files_infolder::{FilelistTp, files_infolder};
use rusqlite::Connection;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// symbols.rs - Symbolic constants for IDOC file conversion to JSON format
// (2021-07-01 bar8tl)
pub const OKAY    : &str = "00";
pub const RC01    : &str = "01";
pub const EDI_DC40: &str = "EDI_DC40";
pub const CONTROL : &str = "CONTROL";
pub const DATA    : &str = "DATA";
pub const SDATA   : &str = "SDATA";
pub const SEGNAM  : &str = "SEGNAM";
pub const SGM     : &str = "SGM";
pub const QUALF   : &str = "QUALF";
pub const SAME    : &str = "SAME";
pub const LOWER   : &str = "LOWER";
pub const UPPER   : &str = "UPPER";

// Flags
pub const OUTCTRL : bool = false;
pub const OUTDATA : bool = false;
pub const OUTSEGM : bool = true;

// types.rs - Data structures used in IDOC file conversion to JSON format
// (2021-07-01 bar8tl)
#[derive(Debug, Clone, Default, Serialize)]
pub struct FieldTp {
  pub key: String,
  pub val: String
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RdataTp {
  pub segmn: String,
  pub qualf: String,
  pub level: usize,
  pub recno: usize,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SdataTp {
  pub instn: usize,
  pub rdata: Vec<RdataTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LdataTp {
  pub sdata: Vec<SdataTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RsegmTp {
  pub segmn: String,
  pub recno: usize,
  pub level: usize,
  pub qlkey: String,
  pub qlval: String,
  pub instn: usize,
  pub field: Vec<FieldTp>,
  pub child: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SsegmTp {
  pub instn: usize,
  pub cntrl: Vec<FieldTp>,
  pub rsegm: Vec<RsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LsegmTp {
  pub ssegm: Vec<SsegmTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SfildTp {
  pub segmn: String,
  pub recno: usize,
  pub level: usize,
  pub qlkey: String,
  pub qlval: String,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RctrlTp {
  pub instn: usize,
  pub field: Vec<FieldTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LctrlTp {
  pub rctrl: Vec<RctrlTp>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CountTp {
  pub segmn: String,
  pub instn: usize
}

// Data structures for functions of conversin to JSON format
#[derive(Debug, Clone, Default)]
pub struct DidocTp {
  pub dbopt: String,
  pub inppt: String,
  pub inpdr: String,
  pub outdr: String,
  pub flide: String,
  pub flnam: String,
  pub flext: String,
  pub idocn: String,
  pub qutdr: String,
  pub recnf: usize,
  pub setno: i32,
  pub recno: usize,
  pub lctrl: LctrlTp, // Control list
  pub sdata: SdataTp, // Dataset
  pub ldata: LdataTp, // Dataset list
  pub rsegm: RsegmTp, // Segment record
  pub ssegm: SsegmTp, // Segmentset
  pub lsegm: LsegmTp, // Segmentset list
  pub sfild: SfildTp,
  pub count: [Vec<CountTp>; 9],
  pub l    : i32,
  pub c1   : i32,
  pub c2   : i32,
  pub c3   : i32,
  pub c4   : i32,
  pub c5   : i32,
  pub c6   : i32,
  pub c7   : i32,
  pub c8   : i32
}

// json_idocs_inbatch.rs - Start batch process to convert IDOC files from fixed size
// (flat file) format to JSON hierarchical format (2021-07-01 bar8tl)
// Command lne: emi step -s json <IDOC-fxdsz-file>
pub fn json_content_inbatch(dbopt: &String, st: StageTp, idoct: &String) {
  let cnn = Connection::open(dbopt).expect("DB Error");
  let flist: Vec<FilelistTp> = files_infolder(&st.wfstp.inpdr, &st.wfstp.inptp,
    idoct);
  for fl in &flist {
    let rtncd = json_content_onefile(&cnn, &st, &fl);
    if st.wfstp.wkflw == "yes" {
      next_stage(&rtncd, &st, &fl);
    }
  }
}

// json_content_onefile.rs - Convert individual IDOC file from fixed size flat
// format to JSON hierarchical format (2021-07-01 bar8tl)
pub fn json_content_onefile(cnn: &Connection, st: &StageTp, fl: &FilelistTp) ->
  String {
  let it: IdoctpTp = get_idoctp(cnn, &st.wfhdr, &fl.flnam);
  let mut d = DidocTp { ..Default::default() };
  //d.dbopt = dbopt.clone();
  d.inpdr = st.wfstp.inpdr.clone();
  d.outdr = st.wfstp.outdr.clone();
  d.inppt = fl.flpth.clone();
  d.flide = fl.flide.clone();
  d.flnam = fl.flnam.clone();
  d.flext = fl.flext.clone();
  d.idocn = it.itype.clone();
  d.setno = -1; // Initialize Instance of data sets in the file
  d.recnf =  0; // Initialize Number of data records in the file
  let mut cnt  : usize = 0;
  let mut first: bool  = true;
  let ifile = File::open(d.inppt.clone()).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let iline = wline.unwrap();
    cnt += 1;
    if cnt == 1usize {
      if &iline[0..8] == EDI_DC40 {
        format_cntrl_record(cnn, &mut d, &iline, &it.itype, CONTROL, &mut first);
      } else {
        println!("IDOC File {} should start with Control Record", d.flide);
        return RC01.to_string()
      }
    } else {
      format_data_record(cnn, &mut d, &iline, &it.itype, DATA);
    }
  }
  if cnt == 0usize {
    println!("Input IDOC file %s is empty: {}", d.flide);
    return RC01.to_string();
  }
  write_json_file(&mut d);
  return OKAY.to_string();
}

// format_cntrl_record - Read Control Record line and prepare JSON output
// (2021-07-01 bar8tl)
pub fn format_cntrl_record(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, rname: &str, first: &mut bool) {
  let mut f    : OutitmTp = OutitmTp{ ..Default::default() };
  let mut rctrl: RctrlTp  = RctrlTp { ..Default::default() };
  if *first {
    *first = false;
  } else {
    write_json_file(d);
  }
  d.recno  = 0; // Inits at Control Record level
  d.l      = -1;
  (d.c1, d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (-1,-1,-1,-1,-1,-1,-1,-1);
  d.setno += 1;
  d.recnf += 1;
  let mut stmt = cnn.prepare("SELECT dname, strps, endps FROM items WHERE idocn=?1
    and rname=?2 order by seqno;").expect("DB Err");
  let mut rows = stmt.query([idocn, &rname.to_string(),]).expect("DB Err");
  while let Some(row) = rows.next().expect("while row failed") {
    f.dname = row.get(0).unwrap();
    f.strps = row.get(1).unwrap();
    f.endps = row.get(2).unwrap();
    let cdval: String = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue
    }
    rctrl.field.push(FieldTp { key: f.dname, val: cdval });
  }
  rctrl.instn = d.setno as usize;
  d.lctrl.rctrl.push(rctrl);
  d.rsegm = RsegmTp { segmn: idocn.to_string(), recno: 0, level: 0,
    qlkey: String::new(), qlval: String::new(), instn: 0, field: Vec::new(),
    child: Vec::new()
  };
}

// read_data_record.rs - Read Data record line and prepare JSON output for pure
// segment metadata portion (2021-07-01 bar8tl)
pub fn format_data_record(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, rname: &str) {
  let mut f    : OutitmTp = OutitmTp { ..Default::default() };
  let mut g    : OutitmTp = OutitmTp { ..Default::default() };
  let mut rdata: RdataTp  = RdataTp  { ..Default::default() };
  d.recnf += 1;
  d.recno += 1;
  let mut stmt = cnn.prepare("SELECT dname, strps, endps FROM items WHERE idocn=?1
    AND rname=?2 order by seqno;").expect("DB Err");
  let mut rows = stmt.query([idocn, &rname.to_string(),]).expect("DB Err");
  while let Some(row) = rows.next().expect("while row failed") {
    f.dname = row.get(0).unwrap();
    f.strps = row.get(1).unwrap();
    f.endps = row.get(2).unwrap();
    if f.endps >= iline.len() {
      f.endps = iline.len();
    }
    let cdval: String = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue
    }
    if f.dname == SEGNAM {
      cnn.query_row("SELECT dname, dtype, dtext, level FROM items WHERE idocn=?1
        AND dname=?2 AND rname=\"SEGMENT\";", [idocn, &cdval,], |row| {
        Ok({
          g.dname = row.get(0).unwrap();
          g.dtype = row.get(1).unwrap();
          g.dtext = row.get(2).unwrap();
          g.level = row.get(3).unwrap();
        })
      }).expect("DB Err");
      rdata.segmn = g.dtype.clone();
      rdata.qualf = g.dtext.clone();
      rdata.level = g.level.clone();
      rdata.recno = d.recno.clone();
    }
    if f.dname == SDATA {
      calc_segmt_counters(cnn, d, iline, idocn, &g.dname, rdata.level);
      continue;
    }
    rdata.field.push(FieldTp{ key: f.dname, val: cdval });
  }
  d.sdata.rdata.push(rdata);
}

// calc_segmt_counters.rs - Process segment data (2021-07-01 bar8tl)
// proc_segmt.rs: Process Segment Data - Determines segment Qualifier and Instance
// Number
pub fn calc_segmt_counters(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, cdnam: &String, level: usize) {
  let mut instn: i32    = -1;
  let mut ident: String = String::new();
  if level == d.l as usize {
    instn = updt_counter(d, cdnam.to_string(), d.l as usize);
    ident = SAME.to_string();
  } else if level > d.l as usize || d.l < 0 {
    d.l = level as i32;
    d.count[d.l as usize].push(CountTp { segmn: cdnam.to_string(), instn: 1 });
    instn = rtrv_counter(d, cdnam.to_string(), d.l as usize);
    ident = LOWER.to_string();
  } else if level < d.l as usize {
    let goupl: usize = d.l as usize - level;
    for _ in 0..goupl {
      d.count[d.l as usize] = Default::default();
      d.l -= 1;
    }
    instn = updt_counter(d, cdnam.to_string(), d.l as usize);
    ident = UPPER.to_string();
  }
  add_tostruct(cnn, d, iline, idocn, ident, cdnam.to_string(), d.l, instn as usize);
}

// updt_counter.rs: Update counter of segment with equal segment ID in the current
// struct level
pub fn updt_counter(d: &mut DidocTp, segmn: String, l: usize) -> i32 {
  for j in 0..d.count[l].len() {
    if d.count[l][j].segmn == segmn {
      d.count[l][j].instn += 1;
      return d.count[l][j].instn as i32;
    }
  }
  d.count[l].push(CountTp{ segmn: segmn, instn: 1 });
  return 1;
}

// rtrv_counter.rs: Retrieve last counter of segment with equal segm ID in the
// current struct lvl
pub fn rtrv_counter(d: &mut DidocTp, segmn: String, l: usize) -> i32 {
  for j in 0..d.count[l].len() {
    if d.count[l][j].segmn == segmn {
      return d.count[l][j].instn as i32;
    }
  }
  return 0;
}

// add_tostruct.rs - Build segment structure into an non-linked segment node
// (2021-07-01 bar8tl)
pub fn add_tostruct(cnn: &Connection, d: &mut DidocTp, iline: &str, idocn: &String,
   _ident: String, segmn: String, l: i32, instn: usize) {
  if d.recno <= 9999 {
    d.sfild.qlkey = "".to_string();
    d.sfild.qlval = "".to_string();
    d.sfild.field = Default::default();
    get_segmt_fields(cnn, d, iline, idocn, SGM.to_string(), &segmn);
    if l == 1 {
      d.rsegm.child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c2, d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1, -1, -1, -1);
      d.c1 += 1;
    } else if l == 2 {
      d.rsegm.child[d.c1 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c3, d.c4, d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1, -1, -1);
      d.c2 += 1;
    } else if l == 3 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c4, d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1, -1);
      d.c3 += 1;
    } else if l == 4 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c5, d.c6, d.c7, d.c8) = (-1, -1, -1, -1);
      d.c4 += 1;
    } else if l == 5 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c6, d.c7, d.c8) = (-1, -1, -1);
      d.c5 += 1;
    } else if l == 6 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c7, d.c8) = (-1, -1);
      d.c6 += 1;
    } else if l == 7 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child[d.c6 as usize].
        child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      (d.c8) = -1;
      d.c7 += 1;
    } else if l == 8 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child[d.c6 as usize].
        child[d.c7 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
      d.c8 += 1;
    } else if l == 9 {
      d.rsegm.child[d.c1 as usize].child[d.c2 as usize].child[d.c3 as usize].
        child[d.c4 as usize].child[d.c5 as usize].child[d.c6 as usize].
        child[d.c7 as usize].child[d.c8 as usize].child.push(RsegmTp {
        segmn: segmn, recno: d.recno, level: l as usize,
        qlkey: d.sfild.qlkey.clone(), qlval: d.sfild.qlval.clone(), instn: instn,
        field: d.sfild.field.clone(), child: Default::default() });
    }
  }
}

// write_json_file.rs - Write JSON output for Control, Data and Segment structures
// (2021-07-01 bar8tl)
pub fn write_json_file(d: &mut DidocTp) {
  d.ldata.sdata.push(SdataTp {
    instn: d.setno as usize,
    rdata: d.sdata.rdata.clone()
  });
  d.ssegm.rsegm.push(
    d.rsegm.clone()
  );
  d.lsegm.ssegm.push(SsegmTp {
    instn: d.setno as usize,
    cntrl: d.lctrl.rctrl[d.setno as usize].field.clone(),
    rsegm: d.ssegm.rsegm.clone()
  });
  let ofnam = format!("{}{}-{}", d.outdr, d.flnam, format!("{}", d.setno));
  if OUTCTRL {
    let mut file = File::create(format!("{}-control.json", ofnam)).expect("error");
    let fctrl = serde_json::to_string_pretty(&d.lctrl).unwrap();
    let bctrl: &[u8] = fctrl.as_bytes();
    file.write_all(&bctrl).unwrap();
  }
  if OUTDATA {
    let mut file = File::create(format!("{}-data.json", ofnam)).expect("error");
    let fdata = serde_json::to_string_pretty(&d.ldata).unwrap();
    let bdata: &[u8] = fdata.as_bytes();
    file.write_all(&bdata).unwrap();
  }
  if OUTSEGM {
    let mut file = File::create(format!("{}-segment.json", ofnam)).expect("error");
    let fsegm = serde_json::to_string_pretty(&d.lsegm).unwrap();
    let bsegm: &[u8] = fsegm.as_bytes();
    file.write_all(&bsegm).unwrap();
  }
  d.sdata.rdata = Default::default();
  d.ldata.sdata = Default::default();
  d.ssegm.rsegm = Default::default();
  d.lsegm.ssegm = Default::default();
}

// get_segmt_fields.rs - Get field values of a segment into the IDOC structure
// (2021-07-01 bar8tl)
pub fn get_segmt_fields(cnn: &Connection, d: &mut DidocTp, iline: &str,
  idocn: &String, strtp: String, cdnam: &String) {
  let mut f    : OutitmTp = Default::default();
  let mut e    : OutstrTp = Default::default();
  let mut fitem: bool     = true;
  let mut stmt = cnn.prepare("SELECT a.idocn, a.level, a.pseqn, a.pdnam, a.pdtyp,
    a.pdqlf, a.cseqn, a.cdnam, a.cdtyp, a.cdqlf, b.dname, b.seqno, b.strps, b.endps
    FROM struc a LEFT JOIN items b ON (a.idocn = b.idocn and a.cdtyp = b.rname)
    WHERE a.idocn=?1 and a.strtp=?2 and a.cdtyp=?3  ORDER BY a.idocn, a.strtp,
    a.pseqn, a.prnam, a.pdnam, b.seqno;").expect("DB Err");
  let mut rows = stmt.query([idocn, &strtp, &cdnam.to_string(),]).expect("DB Err");
  while let Some(row) = rows.next().expect("while row failed") {
    e.idocn = row.get( 0).unwrap();
    e.level = row.get( 1).unwrap();
    e.pseqn = row.get( 2).unwrap();
    e.pdnam = row.get( 3).unwrap();
    e.pdtyp = row.get( 4).unwrap();
    e.pdqlf = row.get( 5).unwrap();
    e.cseqn = row.get( 6).unwrap();
    e.cdnam = row.get( 7).unwrap();
    e.cdtyp = row.get( 8).unwrap();
    e.cdqlf = row.get( 9).unwrap();
    f.dname = row.get(10).unwrap();
    f.seqno = row.get(11).unwrap();
    f.strps = row.get(12).unwrap();
    f.endps = row.get(13).unwrap();
    if f.endps >= iline.len() {
      break;
    }
    let cdval: String = iline[f.strps-1..f.endps].trim().to_string();
    if cdval.len() == 0 || cdval == "" {
      continue;
    }
    if fitem {
      d.sfild.segmn = e.cdtyp;
      d.sfild.recno = d.recno;
      d.sfild.level = e.level;
      if e.cdqlf == QUALF {
        d.sfild.qlkey = f.dname.clone();
        d.sfild.qlval = cdval.clone();
      } else {
        d.sfild.qlkey = String::new();
        d.sfild.qlval = String::new();
      }
      fitem = false;
    }
    d.sfild.field.push(FieldTp { key: f.dname, val: cdval });
  }
}
