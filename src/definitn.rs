// definitn.rs - Read SAP IDoc parser file, and upload IDoc definition detail and
// structure into the repository (2021-07-01 bar8tl)
use rusqlite::Connection;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

// symbols - Symbolic constants for IDOC Definition add function
pub const BEGIN_         : &str = "BEGIN_";
pub const END_           : &str = "END_";
pub const IDOC           : &str = "IDOC";
pub const EXTENSION      : &str = "EXTENSION";
pub const RECORD         : &str = "RECORD";
pub const GROUP          : &str = "GROUP";
pub const SEGMENTTYPE    : &str = "SEGMENTTYPE";
pub const SEGMENT        : &str = "SEGMENT";
pub const FIELDS         : &str = "FIELDS";
pub const BEGIN          : &str = "BEGIN";
pub const END            : &str = "END";
pub const LEVEL          : &str = "LEVEL";
pub const LOOPMIN        : &str = "LOOPMIN";
pub const LOOPMAX        : &str = "LOOPMAX";
pub const QUALIFIED      : &str = "QUALIFIED";
pub const STATUS         : &str = "STATUS";
pub const NAME           : &str = "NAME";
pub const TEXT           : &str = "TEXT";
pub const TYPE           : &str = "TYPE";
pub const LENGTH         : &str = "LENGTH";
pub const FIELD_POS      : &str = "FIELD_POS";
pub const CHARACTER_FIRST: &str = "CHARACTER_FIRST";
pub const CHARACTER_LAST : &str = "CHARACTER_LAST";
pub const QUALF          : &str = "QUALF";
pub const GRP            : &str = "grp";
pub const SGM            : &str = "sgm";

// types - Data structures used in IDOC Definition upload functions
// Data structures for parsing Structured Hierarchical Input
#[derive(Debug, Clone, Default)]
pub struct ReclbTp {
  pub ident: String,
  pub recnm: String,
  pub rectp: String
}

#[derive(Debug, Clone, Default)]
pub struct ParslTp {
  pub label: ReclbTp,
  pub value: String
}

// Data structures for records as in Input
#[derive(Debug, Clone, Default)]
pub struct IdcdfTp {
  pub name:  String,
  pub typi:  String,
  pub cols: [String; 2] // Name, Extn
}

#[derive(Debug, Clone, Default)]
pub struct GrpdfTp {
  pub name:  String,
  pub typi:  String,
  pub seqn:  usize,
  pub cols: [String; 5] // Numb, Levl, Stat, Mnlp, Mxlp
}

#[derive(Debug, Clone, Default)]
pub struct SgmdfTp {
  pub name:  String,
  pub typi:  String,
  pub seqn:  usize,
  pub cols: [String; 7] // Name, Type, Qual, Levl, Stat, Mnlp, Mxlp
}

#[derive(Debug, Clone, Default)]
pub struct FlddfTp {
  pub name:  String,
  pub typi:  String,
  pub clas:  String,
  pub cols: [String; 7] // Name, Text, Type, Lgth, Seqn, Strp, Endp
}

#[derive(Debug, Clone, Default)]
pub struct InpitmTp<'a> {
  pub icol :  Vec<&'a str>, // idoc    columns
  pub gcol :  Vec<&'a str>, // group   columns
  pub scol :  Vec<&'a str>, // segment columns
  pub fcol :  Vec<&'a str>, // Field   columns
  pub stack:  Vec<ParslTp>, // List of ParslTp: Levels stack
  pub lidoc:  Vec<IdcdfTp>, // List of IdcdfTp: Idoc
  pub lgrup:  Vec<GrpdfTp>, // List of GrpdfTp: Grup
  pub lsegm:  Vec<SgmdfTp>, // List of SegdfTp: Segm
  pub lfild:  Vec<FlddfTp>, // List of FlddfTp: Fild
  pub lrecd:  Vec<FlddfTp>, // List of FlddfTp: Fild
  pub colsi: [String; 2],   // Name, Extn
  pub colsg: [String; 5],   // Numb, Levl, Stat, Mnlp, Mxlp
  pub colss: [String; 7],   // Name, Type, Qual, Levl, Stat, Mnlp, Mxlp
  pub colsf: [String; 7],   // Name, Text, Type, Lgth, Seqn, Strp, Endp
  pub colsr: [String; 7],   // Name, Text, Type, Lgth, Seqn, Strp, Endp
  pub l    :  i32,          // Stack level
  pub gseqn:  usize,        // Group   counter
  pub sseqn:  usize         // Segment counter
}

#[derive(Debug, Clone, Default)]
pub struct InpgrpTp {
  pub stack: Vec<KeystTp>,  // List of KeystTp: Levels stack
  pub idocn: String,
  pub strtp: String,
  pub l    : i32,
  pub gseqn: usize
}

#[derive(Debug, Clone, Default)]
pub struct InpsgmTp {
  pub stack:  Vec<KeystTp>, // List of KeystTp: Levels stack
  pub tnode:  KeystTp,
  pub fnode:  KeystTp,
  pub snode:  KeystTp,
  pub idocn:  String,
  pub strtp:  String,
  pub l    :  i32,
  pub sseqn:  usize
}

#[derive(Debug, Clone, Default)]
pub struct KeystTp { // Structure Node Attributes
//    Field:         // IDOC        GROUP      SEGMENT
//------------------------------------------------------
  pub rname: String, // 'IDOC'      'GROUP'    'SEGMENT'
  pub dname: String, // Basic-IDoc  Group#     Segm-ID
  pub dtype: String, // ''          ''         Segm-Type
  pub dqual: String, // ''          ''         'QUAL'
  pub level: usize,  // 0           Level      Level
  pub pseqn: usize,  // 0           auto-gen   auto-gen
  pub seqno: usize   // 0           Group-Seq  Segm-Seq
}

// Data structures for records as in Output
#[derive(Debug, Clone, Default)]
pub struct OutitmTp { // ITEMS fields description (*=key field in DB record)
//    Field:         //  IDOC        GROUP       SEGMENT     SGM-FIELD   RECRD-FIELD
//----------------------------------------------------------------------------------
  pub idocn: String, //* Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name  Ex/Ba-Name
  pub rname: String, //* 'IDOC'      'GROUP'     'SEGMENT'   Segm-ID     'CONTROL'..
  pub dname: String, //* Basic-IDoc  Group#      Segm-ID     Field-Name  Field-Name
  pub rclas: String, //  Basic-IDoc  Group#      Segm-ID     'SEGMENT'   'RECORD'
  pub rtype: String, //  'IDOC'      'GROUP'     'SEGMENT'   'FIELDS'    'FIELDS'
  pub dtype: String, //  ''          ''          Segm-Type    Data-Type   Data-Type
  pub dtext: String, //  Extsn-name  Group#      Qualified   Field-Desc  Field-Desc
  pub level: usize,  //  0           Level       Level       0           0
  pub stats: String, //  ''          Status      Status      ''          ''
  pub minlp: usize,  //  0           Loop-Min    Loop-Min    0           0
  pub maxlp: usize,  //  0           Loop-Max    Loop-Max    0           0
  pub lngth: usize,  //  0           0           0           Length      Length
  pub seqno: usize,  //  0           auto-gen    Auto-gen    Field-Seqn  Field-Seqn
  pub strps: usize,  //  0           0           0           Start-Pos   Start-Pos
  pub endps: usize   //  0           0           0           End-Pos     End-Pos
}

#[derive(Debug, Clone, Default)]
pub struct OutstrTp { // IDoc-Structure Descr (*=key field in DB record)
//    Field:         //  GROUP                   SEGMENT
//-----------------------------------------------------------------------
  pub idocn: String, //* Ex/Ba-Name              Ex/Ba-Name
  pub strtp: String, //* 'GRP'                   'SGM'
  pub level: usize,  //  auto-gen                auto-gen
  // PARENT
  pub prnam: String, //* p.rname='IDOC'/'GROUP'  p.rname='SEGMENT'
  pub pseqn: usize,  //* p.pseqn=autogen         p.pseqn=autogen
  pub pdnam: String, //* p.dname=Group#          p.dname=Segm-ID
  pub pdtyp: String, //  ''                      p.dtype=Segm-Type
  pub pdqlf: String, //  ''                      'QUAL'
  // CHILD
  pub crnam: String, //* c.rname='GROUP          c.rname*=Segm-ID
  pub cseqn: usize,  //* p.seqno=Group-Seq       p.seqno*=Seqno
  pub cdnam: String, //* c.dname=Group#          c.dname*=Segm/Field-Name
  pub cdtyp: String, //  ''                      c.dtype =Segm/Field-Type
  pub cdqlf: String  //  ''                      'QUAL'
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SetgsTp {
  pub inpdr: String,
  pub inptp: String
}

// Main logic to process: 1) Master data for IDoc Items, 2) Structure for groups,
// and 3) Structure for segments
// Command line: edimaps add -d <idoc-parser-file>
pub fn add_definitn(dbopt: &String, refpt: &String) {
  let cnn = Connection::open(&dbopt).expect("DB Open Error");
  let mut ii = InpitmTp { ..Default::default() };
  let mut ig = InpgrpTp { ..Default::default() };
  let mut is = InpsgmTp { ..Default::default() };
  init_items_master(&mut ii);
  init_group_struct(&mut ig);
  init_segmt_struct(&mut is);
  let ifile = File::open(refpt).unwrap();
  let rdr = BufReader::new(ifile);
  for wline in rdr.lines() {
    let wline = wline.unwrap();
    let line  = wline.trim();
    if line.len() > 0 {
      let sline = scan_parserfile_line(line);
      proc_items_master(      &sline, &mut ii);
      proc_group_struct(&cnn, &sline, &mut ig);
      proc_segmt_struct(&cnn, &sline, &mut is);
    }
  }
  prep_items_output(&cnn, &mut ii);
}

fn init_items_master(ii: &mut InpitmTp) {
  ii.icol = vec![EXTENSION];
  ii.gcol = vec![LEVEL, STATUS, LOOPMIN, LOOPMAX];
  ii.scol = vec![SEGMENTTYPE, QUALIFIED, LEVEL, STATUS, LOOPMIN, LOOPMAX];
  ii.fcol = vec![NAME, TEXT, TYPE, LENGTH, FIELD_POS, CHARACTER_FIRST,
    CHARACTER_LAST];
  ii.l = -1;
}

fn init_group_struct(ig: &mut InpgrpTp) {
  ig.strtp = GRP.to_uppercase();
  ig.l = -1;
}

fn init_segmt_struct(is: &mut InpsgmTp) {
  is.strtp = SGM.to_uppercase();
  is.l = -1;
}

// prep_items_output.rs - Format IDOC item definition detail (idoc, group, segment
// and field) in internal database layouts and start creation of item records into
// the local daabase (2021-07-01 bar8tl)
pub fn prep_items_output(cnn: &Connection, ii: &mut InpitmTp) {
  clear_items(cnn, ii.lidoc[0].cols[1].clone());

  // upld_recd(cnn) - Upload IDoc records data
  // /RB04/YP3_DELVRY_RBNA|CONTROL|TABNAM|RECORD|FIELDS|CHARACTER|
  // Name of Table Structure||0|0||0|0|10|1|1|10
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lrecd in &ii.lrecd {
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lrecd.name.clone();    // B…_CONTROL_R…   CONTROL
    w.dname = lrecd.cols[0].clone(); // NAME            TABNAM
    w.rclas = lrecd.clas.clone();    // B…_C…_RECORD    RECORD
    w.rtype = lrecd.typi.clone();    // B…_FIELDS       FIELDS
    w.dtype = lrecd.cols[2].clone(); // TYPE            CHARACTER
    w.dtext = lrecd.cols[1].clone(); // TEXT            Name of Table Stru…
    w.level = 0;
    w.stats = Default::default();
    w.minlp = 0;
    w.maxlp = 0;
    w.lngth = lrecd.cols[3].parse::<usize>().unwrap(); // LENGTH          000010
    w.seqno = lrecd.cols[4].parse::<usize>().unwrap(); // FIELD_POS       0001
    w.strps = lrecd.cols[5].parse::<usize>().unwrap(); // CHARACTER_FIRST 000001
    w.endps = lrecd.cols[6].parse::<usize>().unwrap(); // CHARACTER_LAST  000010
    write_items(cnn, w.clone());
  }

  // upld_idoc(cnn) - Upload IDoc idoc data
  // /RB04/YP3_DELVRY_RBNA|IDOC|DELVRY07|DELVRY07|IDOC|||/RB04/YP3_DELVRY_RBNA|0|
  // 0||0|0|0|0|0|0
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lidoc in &ii.lidoc {
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lidoc.typi.clone();    // B…_IDOC         IDOC
    w.dname = lidoc.cols[0].clone(); // BEGIN_IDOC      DELVRY07
    w.rclas = lidoc.name.clone();    // BEGIN_IDOC      DELVRY07
    w.rtype = lidoc.typi.clone();    // B…_IDOC         IDOC
    w.dtype = Default::default();
    w.dtext = lidoc.cols[1].clone(); // EXTENSION       /RB04/YP3_DELVRY_RBNA
    w.level = 0;
    w.stats = Default::default();
    w.minlp = 0;
    w.maxlp = 0;
    w.lngth = 0;
    w.seqno = 0;
    w.strps = 0;
    w.endps = 0;
    write_items(cnn, w.clone());
  }

  // upld_grup(cnn) - Upload IDoc groups data
  // /RB04/YP3_DELVRY_RBNA|GROUP|1|1|GROUP||||1|2|MANDATORY|1|9999|0|0|0|0
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lgrup in &ii.lgrup {
    ii.gseqn += 1;
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lgrup.typi.clone();    // B…_GROUP        GROUP
    w.dname = lgrup.cols[0].clone(); // BEGIN_GROUP     1
    w.rclas = lgrup.name.clone();    // BEGIN_GROUP     1
    w.rtype = lgrup.typi.clone();    // B…_GROUP        GROUP
    w.dtype = Default::default();
    w.dtext = lgrup.cols[0].clone(); // BEGIN_GROUP     1
    w.level = lgrup.cols[1].parse::<usize>().unwrap(); // LEVEL       02
    w.stats = lgrup.cols[2].clone();                   // STATUS      MANDATORY
    w.minlp = lgrup.cols[3].parse::<usize>().unwrap(); // LOOPMIN     0000000001
    w.maxlp = lgrup.cols[4].parse::<usize>().unwrap(); // LOOPMAX     0000009999
    w.lngth = 0;
    w.seqno = lgrup.seqn.clone();
    w.strps = 0;
    w.endps = 0;
    write_items(cnn, w.clone());
  }

  // upld_segm(cnn) - Upload IDoc segments data
  // /RB04/YP3_DELVRY_RBNA|SEGMENT|E2EDL20004|E2EDL20004|SEGMENT|E1EDL20|QUAL||
  // 0|2|MANDATORY|1|1|0|0|0|0
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lsegm in &ii.lsegm {
    ii.sseqn += 1;
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lsegm.typi.clone();    // B…_SEGMENT      SEGMENT
    w.dname = lsegm.cols[0].clone(); // BEGIN_SEGMENT   E2EDL20004
    w.rclas = lsegm.name.clone();    // BEGIN_SEGMENT   E2EDL20004
    w.rtype = lsegm.typi.clone();    // B…_SEGMENT      SEGMENT
    w.dtype = lsegm.cols[1].clone(); // SEGMENTTYPE     E1EDL20
    w.dtext = lsegm.cols[2].clone(); // QUALIFIED       QUAL
    w.level = lsegm.cols[3].parse::<usize>().unwrap(); // LEVEL       02
    w.stats = lsegm.cols[4].clone();                   // STATUS      MANDATORY
    w.minlp = lsegm.cols[5].parse::<usize>().unwrap(); // LOOPMIN     0000000001
    w.maxlp = lsegm.cols[6].parse::<usize>().unwrap(); // LOOPMAX     0000000001
    w.lngth = 0;
    w.seqno = lsegm.seqn.clone();
    w.strps = 0;
    w.endps = 0;
    write_items(cnn, w.clone());
  }

  // upld_flds(cnn) - Upload IDoc fields data
  // /RB04/YP3_DELVRY_RBNA|E2EDL20004|VKBUR|SEGMENT|FIELDS|CHARACTER|
  // Sales Office||0|0||0|0|4|5|84|87
  let mut w: OutitmTp = OutitmTp { ..Default::default() };
  for lfild in &ii.lfild {
    w.idocn = ii.lidoc[0].cols[1].clone(); // EXTENSION/BASIC /RB04/YP3_DELVRY_RBNA
    w.rname = lfild.name.clone();    // BEGIN_SEGMENT   E2EDL20004
    w.dname = lfild.cols[0].clone(); // NAME            VKBUR
    w.rclas = lfild.clas.clone();    // B…_SEGMENT      SEGMENT
    w.rtype = lfild.typi.clone();    // B…_FIELDS       FIELDS
    w.dtype = lfild.cols[2].clone(); // TYPE            CHARACTER
    w.dtext = lfild.cols[1].clone(); // TEXT            Sales Office
    w.level = 0;
    w.stats = Default::default();
    w.minlp = 0;
    w.maxlp = 0;
    w.lngth = lfild.cols[3].parse::<usize>().unwrap(); // LENGTH          000004
    w.seqno = lfild.cols[4].parse::<usize>().unwrap(); // FIELD_POS       0005
    w.strps = lfild.cols[5].parse::<usize>().unwrap(); // CHARACTER_FIRST 000084
    w.endps = lfild.cols[6].parse::<usize>().unwrap(); // CHARACTER_LAST  000087
    write_items(cnn, w.clone());
  }
}

// scan_parserfile_line.rs - Identify individual tokens in SAP IDOC data in parser
// file format (2021-07-01 bar8tl)
pub fn scan_parserfile_line(s: &str) -> ParslTp {
  let key: String;
  let mut val: String;
  let mut p = ParslTp { ..Default::default() };
  let flds: Vec<&str> = s.split_whitespace().collect();
  if flds.len() > 0 {
    key = flds[0].to_string();
    if (key.len() >= 6 && &key[0..6] == BEGIN_) ||
       (key.len() >= 4 && &key[0..4] == END_  ) {
      let tokn: Vec<&str> = key.split('_').collect();
      if tokn.len() == 2 {
        p.label.ident = tokn[0].to_string();
        p.label.recnm = tokn[1].to_string();
        p.label.rectp = Default::default();
      } else if tokn.len() == 3 {
        p.label.ident = tokn[0].to_string();
        p.label.recnm = tokn[1].to_string();
        p.label.rectp = tokn[2].to_string();
      }
    } else {
      p.label.ident = key;
      p.label.recnm = String::new();
      p.label.rectp = String::new();
    }
  }
  if flds.len() > 1 {
    val = flds[1].to_string();
    for i in 2..flds.len() {
      val = format!("{} {}", val, flds[i]);
    }
    p.value = val;
  }
  return p;
}

// proc_group_struct.rs - Get IDOC groups structure detail and start creation of
// corresponding structure records into the local database (2021-07-01 bar8tl)
pub fn proc_group_struct(cnn: &Connection, sline: &ParslTp, ig: &mut InpgrpTp) {
  if sline.label.ident == BEGIN {
    if sline.label.recnm == IDOC {
      ig.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: String::new(), dqual: String::new(),
        level: 0, pseqn: 0, seqno: 0
      });
      ig.l += 1;
      ig.idocn = sline.value.clone();
      clear_struc(cnn, ig.idocn.clone(), ig.strtp.clone());
    } else if sline.label.recnm == GROUP {
      ig.stack[ig.l as usize].seqno += 1;
      ig.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: String::new(), dqual: String::new(),
        level: 0, pseqn: 0, seqno: 0
      });
      ig.l += 1;
    }
    return;
  }
  if sline.label.ident == END {
    if sline.label.recnm == IDOC {
      ig.stack = ig.stack[..ig.l as usize].to_vec();
      ig.l -= 1;
    } else if sline.label.recnm == GROUP {
      ig.gseqn += 1;
      ig.stack[ig.l as usize-1].pseqn = ig.gseqn;
      write_struc(cnn, ig.idocn.clone(), ig.strtp.clone(),
        ig.stack[ig.l as usize-1].clone(), ig.stack[ig.l as usize].clone());
      ig.stack = ig.stack[..ig.l as usize].to_vec();
      ig.l -= 1;
    }
    return;
  }
  if ig.l >= 0 && ig.stack[ig.l as usize].rname == IDOC {
    if sline.label.ident == EXTENSION {
      ig.idocn = sline.value.clone();
      clear_struc(cnn, ig.idocn.clone(), ig.strtp.clone());
    }
    return;
  }
}

// proc_segmt_struct.rs - Get IDOC segments structure data and start creation of
// corresponding structure records into the local database (2021-07-01 bar8tl)
pub fn proc_segmt_struct(cnn: &Connection, sline: &ParslTp, is: &mut InpsgmTp) {
  if sline.label.ident == BEGIN {
    if sline.label.recnm == IDOC {
      is.stack.push(KeystTp {
        rname: sline.label.recnm.clone(),
        dname: sline.value.clone(),
        dtype: Default::default(), dqual: Default::default(),
        level: 0, pseqn: 0, seqno: 0
      });
      is.l += 1;
      is.tnode.rname = sline.label.recnm.clone();
      is.tnode.dname = sline.value.clone();
      is.tnode.dqual = Default::default();
      is.tnode.pseqn = 0;
      is.idocn       = sline.value.clone();
      clear_struc(cnn, is.idocn.clone(), is.strtp.clone());
    } else if sline.label.recnm == SEGMENT && sline.label.rectp.len() == 0 {
      is.sseqn += 1;
      is.tnode.rname = sline.label.recnm.clone();
      is.tnode.dname = sline.value.clone();
      is.tnode.dqual = Default::default();
      is.tnode.pseqn = is.sseqn.clone();
    }
    return;
  }

  if sline.label.ident == END && is.l >= 0 {
    if sline.label.recnm == IDOC {
      is.stack = is.stack[..is.l as usize].to_vec();
      is.l -= 1;
    } else if sline.label.recnm == SEGMENT && sline.label.rectp.len() == 0 {
      if is.l == 0 {
        is.stack[is.l as usize].seqno += 1;
        is.stack.push(KeystTp {
          rname: is.tnode.rname.clone(),
          dname: is.tnode.dname.clone(),
          dtype: is.tnode.dtype.clone(),
          dqual: is.tnode.dqual.clone(),
          level: is.tnode.level.clone(),
          pseqn: is.tnode.pseqn.clone(),
          seqno: 0
        });
        is.l += 1;
      } else if is.tnode.level <= is.stack[is.l as usize].level {
        while is.tnode.level <= is.stack[is.l as usize].level {
          write_struc(cnn, is.idocn.clone(), is.strtp.clone(),
            is.stack[is.l as usize-1].clone(),
            is.stack[is.l as usize  ].clone());
          is.stack = is.stack[..is.l as usize].to_vec();
          is.l -= 1;
        }
        is.stack[is.l as usize].seqno += 1;
        is.stack.push(KeystTp {
          rname: is.tnode.rname.clone(),
          dname: is.tnode.dname.clone(),
          dtype: is.tnode.dtype.clone(),
          dqual: is.tnode.dqual.clone(),
          level: is.tnode.level.clone(),
          pseqn: is.tnode.pseqn.clone(),
          seqno: 0
        });
        is.l += 1;
      } else if is.tnode.level > is.stack[is.l as usize].level {
        is.stack[is.l as usize].seqno += 1;
        is.stack.push(KeystTp {
          rname: is.tnode.rname.clone(),
          dname: is.tnode.dname.clone(),
          dtype: is.tnode.dtype.clone(),
          dqual: is.tnode.dqual.clone(),
          level: is.tnode.level.clone(),
          pseqn: is.tnode.pseqn.clone(),
          seqno: 0
        });
        is.l += 1;
      }
    } else if sline.label.recnm == FIELDS && is.l >= 0 {
      is.fnode.rname = Default::default();
      is.fnode.dname = Default::default();
      is.fnode.dqual = Default::default();
    }
    return;
  }

  if is.tnode.rname == SEGMENT && is.tnode.dname.len() > 0 {
    if sline.label.ident == SEGMENTTYPE {
      is.tnode.dtype = sline.value.clone();
    }
    if sline.label.ident == QUALIFIED {
      is.tnode.dqual = QUALF.to_string();
    }
    if sline.label.ident == LEVEL {
      let l = sline.value.parse::<usize>().unwrap();
      is.tnode.level = l;
    }
    return;
  }

  if is.tnode.rname == IDOC {
    if sline.label.ident == EXTENSION {
      is.idocn = sline.value.clone();
      clear_struc(cnn, is.idocn.clone(), is.strtp.clone());
    }
    return;
  }
}

// proc_items_master.rs - Get IDOC item detail (records, groups, segments and
// fields) and start creation of corresponding item records into the local database
// (2021-07-01 bar8tl)
// Scan SAP parser file to identify IDoc elements
pub fn proc_items_master(sline: &ParslTp, ii: &mut InpitmTp) {
  if sline.label.ident == BEGIN {
    ii.l += 1;
    ii.stack.push(ParslTp { label: ReclbTp { ident: sline.label.ident.clone(),
      recnm: sline.label.recnm.clone(), rectp: sline.label.rectp.clone() },
      value: sline.value.clone() });
    if sline.value != "" {
      if sline.label.recnm == IDOC {
        ii.colsi[0] = sline.value.clone();
        ii.colsi[1] = sline.value.clone();
        ii.lidoc.push(IdcdfTp {
          name: ii.colsi[0].clone(),
          typi: ii.stack[ii.l as usize].label.recnm.clone(),
          cols: ii.colsi.clone()
        });
      } else if sline.label.recnm == GROUP   {
        ii.colsg[0] = sline.value.clone();
      } else if sline.label.recnm == SEGMENT {
        ii.colss[0] = sline.value.clone();
        ii.colss[2] = String::new();
      }
    }
    return;
  }

  if sline.label.ident == END {
    ii.l -= 1;
    if ii.l < 0 {
      ii.stack = Default::default();
    } else {
      ii.stack = ii.stack[..ii.l as usize+1].to_vec();
    }
    return;
  }

  if ii.stack[ii.l as usize].label.recnm == IDOC {
    for i in 0..ii.icol.len() {
      if sline.label.ident == ii.icol[i] {
        ii.colsi[i+1] = sline.value.clone();
        if i == ii.icol.len() - 1 {
          ii.lidoc[0].cols[1] = ii.colsi[i+1].clone();
        }
        break;
      }
    }
  }

  if ii.stack[ii.l as usize].label.recnm == GROUP {
    for i in 0..ii.gcol.len() {
      if sline.label.ident == ii.gcol[i] {
        ii.colsg[i+1] = sline.value.clone();
        if i == ii.gcol.len() - 1 {
          ii.gseqn += 1;
          ii.lgrup.push(GrpdfTp {
            name: ii.colsg[0].clone(),
            typi: ii.stack[ii.l as usize].label.recnm.clone(),
            seqn: ii.gseqn.clone(),
            cols: ii.colsg.clone()
          });
        }
        break;
      }
    }
  }

  if ii.stack[ii.l as usize].label.recnm == SEGMENT {
    for i in 0..ii.scol.len() {
      if sline.label.ident == ii.scol[i] {
        if sline.label.ident == QUALIFIED {
          ii.colss[i+1] = QUALF.to_string();
        } else {
          ii.colss[i+1] = sline.value.clone();
        }
        if i == ii.scol.len() - 1 {
          ii.sseqn += 1;
          ii.lsegm.push(SgmdfTp {
            name: ii.colss[0].clone(),
            typi: ii.stack[ii.l as usize].label.recnm.clone(),
            seqn: ii.sseqn.clone(),
            cols: ii.colss.clone()
          });
        }
        break;
      }
    }
  }

  if ii.stack[ii.l as usize].label.recnm == FIELDS {
    let mut mtch = false;
    for i in 0..ii.fcol.len() {
      if sline.label.ident == ii.fcol[i] {
        ii.colsf[i] = sline.value.clone();
        mtch = true;
      }
      if i == ii.fcol.len()-1 {
        if ii.stack[ii.l as usize-1].label.rectp == RECORD {
          ii.lrecd.push(FlddfTp {
            name: ii.stack[ii.l as usize-1].label.recnm.clone(),
            typi: ii.stack[ii.l as usize  ].label.recnm.clone(),
            clas: ii.stack[ii.l as usize-1].label.rectp.clone(),
            cols: ii.colsf.clone()
          });
        } else if ii.stack[ii.l as usize-1].label.recnm == SEGMENT {
          ii.lfild.push(FlddfTp{
            name: ii.colss[0].clone(),
            typi: ii.stack[ii.l as usize  ].label.recnm.clone(),
            clas: ii.stack[ii.l as usize-1].label.recnm.clone(),
            cols: ii.colsf.clone()
          });
        }
      }
      if mtch {
        break;
      }
    }
  }
}

// write_items_indb.rs - Functions to clear/write IDOC item detail records (idoc,
// group, segment and field) into the local DB (2021-07-01 bar8tl)
pub fn write_items(cnn: &Connection, w: OutitmTp) {
  cnn.execute(
    "INSERT INTO items VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
    (w.idocn, w.rname, w.dname, w.rclas, w.rtype, w.dtype, w.dtext, w.level,
     w.stats, w.minlp, w.maxlp, w.lngth, w.seqno, w.strps, w.endps,))
    .expect("Items insertion error");
}

pub fn clear_items(cnn: &Connection, idocn: String) {
  cnn.execute("DELETE FROM items WHERE idocn=?1", (idocn,))
    .expect("Items clearing error");
}

// write_struc_indb.rs - Functions to clear/write IDOC structure records (idoc,
// group and segment levels) into the local DB (2021-07-01 bar8tl)
pub fn write_struc(cnn: &Connection, idocn: String, strtp: String, pnode: KeystTp,
  cnode: KeystTp) {
  let mut pdnam = String::new();
  let mut cdnam = String::new();
  if strtp == GRP.to_uppercase() {
    let test = pnode.dname.parse::<usize>();
    match test {
      Ok(_ok) => pdnam = format!("{:02}", pnode.dname.parse::<usize>().unwrap()),
      Err(_e) => pdnam = pnode.dname.clone(),
    }
    cdnam = format!("{:02}", cnode.dname.parse::<usize>().unwrap());
  }
  cnn.execute(
    "INSERT INTO struc VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
    (idocn, strtp,       pnode.level, pnode.rname, pnode.pseqn,
     pdnam, pnode.dname, pnode.dqual, cnode.rname, pnode.seqno,
     cdnam, cnode.dname, cnode.dqual,))
    .expect("Struc insertion error");
}

pub fn clear_struc(cnn: &Connection, idocn: String, strtp: String) {
  cnn.execute("DELETE FROM struc WHERE idocn=?1 and strtp=?2", (idocn, strtp,))
    .expect("Struc clearing error");
}
