// tofixdsz.rs - Starts proper function to convert IDOC content from structured
// hierarchical format to fixed size format. Either from a set of files contained
// within a folder or from an specific single file (2021-07-01 bar8tl)
use crate::assets::IdoctpTp;
use crate::readidoc::read_idocs::next_stage;
use crate::readidoc::read_idocs::{StageTp, get_idoctp};
use rblib::files_infolder::{FilelistTp, files_infolder};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

// symbols.rs - Symbolic constants for IDOC file conversion to fixed size format
// (2021-07-01 bar8tl)
pub const OKAY   : &str = "00";
pub const EDIDC  : &str = "EDIDC";
pub const EDIDD  : &str = "EDIDD";
pub const EDIDS  : &str = "EDIDS";
pub const SEGNUM : &str = "SEGNUM";
pub const SEGNAM : &str = "SEGNAM";
pub const DATA   : &str = "DATA";
pub const MANDT  : &str = "MANDT";
pub const DOCNUM : &str = "DOCNUM";
pub const PSGNUM : &str = "PSGNUM";
pub const HLEVEL : &str = "HLEVEL";
pub const TABNAM : &str = "TABNAM";
pub const RCVPFC : &str = "RCVPFC";
pub const SERIAL : &str = "SERIAL";
pub const RCVPRN : &str = "RCVPRN";
pub const RVCPRN : &str = "RVCPRN";
pub const CREDAT : &str = "CREDAT";
pub const CRETIM : &str = "CRETIM";
pub const IDOCTYP: &str = "IDOCTYP";
pub const CIMTYP : &str = "CIMTYP";

// types.rs - Data structures used in IDOC file conversion to fixed size format
// (2021-07-01 bar8tl)
#[derive(Debug, Clone, Default)]
pub struct HstrucTp {
  pub sgnum: String,
  pub sgnam: String,
  pub sglvl: String
}

#[derive(Debug, Clone, Default)]
pub struct ConvertTp {
  pub cntrl: String,
  pub clien: String,
  pub inpdr: String,
  pub outdr: String,
  pub rcvpf: String,
  pub idocx: String,
  pub idocn: String,
  pub idocb: String,
  pub sectn: String,
  pub secnb: String,
  pub sgnum: String,
  pub sgnam: String,
  pub sgdsc: String,
  pub sgnbk: String,
  pub sghnb: String,
  pub sglvl: String,
  pub serie: String,
  pub nsegm: usize,
  pub dirty: bool,
  pub parnt: Vec<HstrucTp>,
  pub l    : usize
}

// flat_content_inbatch.rs - Start batch process to convert IDOC files from
// structured hierarchical (parser file) format to fixed-size (flat) format
// (2021-07-01 bar8tl)
// Command line: emi step -s fixed <Idoc-hierachical-file>
pub fn flat_content_inbatch(dbopt: &String, st: StageTp, idoct: &String) {
  let cnn = Connection::open(dbopt).expect("DB Error");
  let flist: Vec<FilelistTp> = files_infolder(&st.wfstp.inpdr, &st.wfstp.inptp,
    idoct);
  for fl in &flist {
    let rtncd = flat_content_onefile(&cnn, &st, &fl);
    if st.wfstp.wkflw == "yes" {
      next_stage(&rtncd, &st, &fl);
    }
  }
}

// flat_idocs_onefile.rs - Convert individual IDOC file from classic hierarchical
// format to flat text file format (2021-07-01 bar8tl)
pub fn flat_content_onefile(cnn: &Connection, st: &StageTp, fl: &FilelistTp) ->
  String {
  let it: IdoctpTp = get_idoctp(cnn, &st.wfhdr, &fl.flnam);
  let mut c = ConvertTp { ..Default::default() };
  c.cntrl = it.cntrl.clone();
  c.clien = it.clien.clone();
  c.rcvpf = it.rcvpf.clone();
  c.inpdr = st.wfstp.inpdr.clone();
  c.outdr = st.wfstp.outdr.clone();
  c.idocx = it.itype.to_uppercase();
  c.idocb = get_idoc_basicid(cnn, &c.idocx);
  c.parnt.push(HstrucTp { .. Default::default() });
  let mut lctrl = [' ';  524];
  let mut lsegm = [' '; 1063];
  let mut lstat = [' ';  562];
  let mut of = File::create(format!("{}{}.{}", st.wfstp.outdr, fl.flnam,
    st.wfstp.outtp)).expect("creation failed");
  let ifile = File::open(&fl.flpth).unwrap();
  let rdr = BufReader::new(ifile);
  for wlin in rdr.lines() {
    let wlin = wlin.unwrap();
    let line = wlin.trim();
    let tokn: Vec<&str> = line.split('\t').collect();
    if line.len() == 0 { // ignores lines in blank
      continue;
    }

    // Gets IDoc number
    if c.idocn.len() == 0 && tokn.len() == 1 &&
       line[0..11] == "IDoc Number".to_string() {
      let idtkn: Vec<&str> = line.split(" : ").collect();
      c.idocn = idtkn[1].trim().to_string();
      continue;
    }

    // Ignores lines no containing tabulators (after to have gotten IDoc number)
    if tokn.len() <= 1 {
      continue
    }

    // Determines data section to analyze
    if tokn[0] == EDIDC || tokn[0] == EDIDD || tokn[0] == EDIDS {
      prep_sectn_header(cnn, &mut c, &mut lctrl, &mut lsegm, &mut lstat, tokn,
        &mut of);
      continue;
    }

    // Checks in segment number to analize
    if tokn[0] == SEGNUM && tokn.len() == 3 {
      c.sgnbk = c.sgnum.clone();
      c.sgnum = tokn[2].to_string();
      continue;
    }

    // Checks in segment name to analize
    if tokn[0] == SEGNAM && tokn.len() == 3 {
      prep_segmt_header(cnn, &mut c, &mut lsegm, tokn, &mut of);
      continue;
    }

    // Process fields of each data section
    if c.sectn == EDIDC {
      build_edidc_line(cnn, &mut c, &mut lctrl, tokn);
    } else if c.sectn == EDIDD {
      build_edidd_line(cnn, &mut c, &mut lsegm, tokn);
    } else if c.sectn == EDIDS {
      build_edids_line();
    }
  }
  return OKAY.to_string();
}

// prep_sectn_header.rs - Function to prepare measures to take for each data
// section. Each new section causes dumping data from previous one
// (2021-07-01 bar8tl)
pub fn prep_sectn_header(cnn: &Connection, c: &mut ConvertTp,
   lctrl: &mut [char;  524], lsegm: &mut [char; 1063], lstat: &mut [char;  562],
   tokn: Vec<&str>, of: &mut File) {
  c.sectn = tokn[0].to_string();
  if c.sectn == EDIDC {
    *lctrl = [' '; 524];
  }
  if c.sectn == EDIDD {
    write_cntrl_line(cnn, c, lctrl, of);
  }
  if c.sectn == EDIDS {
    c.sgnbk = c.sgnum.clone();
    write_segmt_line(cnn, c, lsegm, of);
    *lstat = [' '; 562];
    if tokn.len() == 3 {
      c.secnb = tokn[2].to_string();
    }
  }
}

// prep_segmt_header.rs - Function to prepare measures to take for each data segment
// in Data Idoc being converted (2021-07-01 bar8tl)
pub fn prep_segmt_header(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], tokn: Vec<&str>, of: &mut File) {
  c.nsegm += 1;
  if c.nsegm > 1 {
    write_segmt_line(cnn, c, lsegm, of);
  }
  c.sgnam = tokn[2].to_string();
  *lsegm = [' '; 1063];
  let mut level: usize = 0;
  cnn.query_row("SELECT dname, level FROM items WHERE idocn=?1 and rname=\"SEGMENT\"
    and dtype=?2;", [c.idocx.clone(), c.sgnam.clone()], |row| {
      Ok({
        c.sgdsc = row.get(0).unwrap();
        level   = row.get(1).unwrap();}
      )
    }).expect("Error: Idoc type not found in definition DB"
  );
  c.sglvl = format!("{:02}", level);

  if c.nsegm == 1 {
    c.parnt.push(HstrucTp{ sgnum: c.sgnum.clone(), sgnam: c.sgnam.clone(),
      sglvl: c.sglvl.clone() });
    c.l += 1;
    c.sghnb = "000000".to_string();
  } else {
    if c.sglvl > c.parnt[c.l].sglvl {
      c.parnt.push(HstrucTp{ sgnum: c.sgnum.clone(), sgnam: c.sgnam.clone(),
       sglvl: c.sglvl.clone() });
      c.l += 1;
      c.sghnb = c.parnt[c.l-1].sgnum.clone();
    } else if c.sglvl == c.parnt[c.l].sglvl {
      c.parnt[c.l].sgnum = c.sgnum.clone();
      c.parnt[c.l].sgnam = c.sgnam.clone();
      c.parnt[c.l].sglvl = c.sglvl.clone();
      c.sghnb = c.parnt[c.l-1].sgnum.clone();
    } else {
      let prvlv = c.parnt[c.l].sglvl.parse::<usize>().unwrap();
      let curlv = c.sglvl.           parse::<usize>().unwrap();
      let nstep = prvlv - curlv;
      for _ in 1..nstep {
        c.l -= 1;
        c.parnt = c.parnt[..c.l+1].to_vec();
      }
      c.parnt[c.l].sgnum = c.sgnum.clone();
      c.parnt[c.l].sgnam = c.sgnam.clone();
      c.parnt[c.l].sglvl = c.sglvl.clone();
      c.sghnb = c.parnt[c.l-1].sgnum.clone();
    }
  }
}

// build_edidc_line.rs - Build cumulatively the Control Record (EDIDC) output
// line (2021-07-01 bar8tl)
pub fn build_edidc_line(cnn: &Connection, c: &mut ConvertTp,
  lctrl: &mut [char; 524], tokn: Vec<&str>) {
  let mut flkey = tokn[0];
  if flkey == RVCPRN {
    flkey = RCVPRN;
  }
  let mut flval: String = Default::default();
  if tokn.len() == 3 {
    let flds: Vec<&str> = tokn[2].split(" :").collect();
    flval = flds[0].to_string();
  }
  if flkey == CREDAT {
    c.serie = flval.clone();
  }
  if flkey == CRETIM {
    c.serie = format!("{}{}", c.serie, flval);
  }
  if flval.len() > 0 {
    c.dirty = true;
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, flkey, flval);
  }
}

// build_edidd_line.rs - Build cumulatively the Data Segment (EDIDD) output
// line (2021-07-01 bar8tl)
pub fn build_edidd_line(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], tokn: Vec<&str>) {
  let flkey = tokn[0];
  let mut flval = Default::default();
  if tokn.len() == 3 {
    let flds: Vec<&str> = tokn[2].split(" :").collect();
    flval = flds[0].to_string();
  }
  if flval.len() > 0 {
    c.dirty = true;
    let sgdsc = c.sgdsc.clone();
    append_field_tosegmt(cnn, &c.idocx, lsegm, sgdsc.as_str(), flkey, flval);
  }
}

// build_edids_line.rs - Build cumulatively the Status Record (EDIDC) output
// line (2021-07-01 bar8tl)
pub fn build_edids_line() {
  /* No needed in systems communication */
}

// write_cntrl_line.rs - Complete output of control record line and address it to a
// flat fixed size text file (2021-07-01 bar8tl)
pub fn write_cntrl_line(cnn: &Connection, c: &mut ConvertTp,
  lctrl: &mut [char; 524], of: &mut File) {
  if c.dirty {
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, TABNAM, c.cntrl.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, MANDT , c.clien.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, DOCNUM, c.idocn.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, RCVPFC, c.rcvpf.clone());
    append_field_tocntrl(cnn, &c.idocx, &c.idocb, lctrl, SERIAL, c.serie.clone());
    let oline: String = lctrl.iter().collect();
    of.write_all(format!("{}\r\n", oline).as_bytes()).expect("write failed");
    c.dirty = false;
  }
}

// write_segmt_line.rs - Complete output of data segment lines and address it to a
// flat fixed size text file (2021-07-01 bar8tl)
pub fn write_segmt_line(cnn: &Connection, c: &mut ConvertTp,
  lsegm: &mut [char; 1063], of: &mut File) {
  if c.dirty {
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, SEGNAM, c.sgdsc.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, MANDT , c.clien.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, DOCNUM, c.idocn.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, SEGNUM, c.sgnbk.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, PSGNUM, c.sghnb.clone());
    append_field_tosegmt(cnn, &c.idocx, lsegm, DATA, HLEVEL, c.sglvl.clone());
    let oline: String = lsegm.iter().collect();
    of.write_all(format!("{}\r\n", oline).as_bytes()).expect("write failed");
    c.dirty = false;
  }
}

// append_field_tocntrl.rs - Append a new field value to the output control record
// line (2021-07-01 bar8tl)
pub fn append_field_tocntrl(cnn: &Connection, idocx: &String, idocb: &String,
  lctrl: &mut [char; 524], flkey: &str, mut flval: String) {
  let mut strps: usize = 0;
  cnn.query_row("SELECT strps FROM items WHERE idocn=?1 and rname=\"CONTROL\"
    and dname=?2;", [idocx.to_string(), flkey.to_string()], |row| {
      Ok(strps = row.get(0).unwrap()) })
    .expect("Error: Idoc type not found in definition DB");
  if flkey == IDOCTYP && flval == "14" {
    flval = idocb.to_string();
  }
  if flkey == CIMTYP  && flval == "14" {
    flval = idocx.to_string();
  }
  let mut k: usize = strps - 1;
  let temp: Vec<char> = flval.chars().collect();
  for i in 0..temp.len() {
    lctrl[k] = temp[i];
    k += 1;
  }
}

// append_field_tosegmt.rs - Append a new field value to the output segment line
// (2021-07-01 bar8tl)
pub fn append_field_tosegmt(cnn: &Connection, idocx: &String,
  lsegm: &mut [char; 1063], sgdsc: &str, flkey: &str, flval: String) {
  let mut strps: usize = 0;
  cnn.query_row("SELECT strps FROM items WHERE idocn=?1 and rname=?2 and dname=?3;",
    [idocx.clone(), sgdsc.to_string(), flkey.to_string()], |row| {
      Ok(strps = row.get(0).unwrap()) })
    .expect("Error: Segment type not found in definition DB");
  let mut k: usize = strps - 1;
  let temp: Vec<char> = flval.chars().collect();
  for i in 0..temp.len() {
    lsegm[k] = temp[i];
    k += 1;
  }
}

// get_idoc_basicid.rs - Retrieve the basi name of the IDOC type that is being
// converted (2021-07-01 bar8tl)
pub fn get_idoc_basicid(cnn: &Connection, idocx: &String) -> String {
  let mut idocb: String = Default::default();
  cnn.query_row("SELECT dname FROM items WHERE idocn=?1 and rname=\"IDOC\";",
    [idocx.to_uppercase()], |row| { Ok(idocb = row.get(0).unwrap()) })
    .expect("Error: Idoc type not found in definition DB");
  return idocb;
}
