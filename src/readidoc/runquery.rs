// query_content.rs - Starts proper function to perform IDOC content inquiry from
// JSON format. Either from a set of files contained within a folder or from an
// specific single file (2021-07-01 bar8tl)
#![allow(unused_variables, unused_imports)]
#![allow(dead_code)]

use crate::assets::IdoctpTp;
use crate::readidoc::read_idocs::next_stage;
use crate::readidoc::read_idocs::{StageTp, get_idoctp};
use crate::readidoc::tojson::{FieldTp, OKAY};
use rblib::files_infolder::{FilelistTp, files_infolder};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io::Write;

// types.rs - Data structures used in IDOC query from files stored in JSON format
// (2021-07-01 bar8tl)
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RquryTp {
  pub fields: Vec<String>
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SquryTp {
  pub fields: Vec<FieldTp>
}

#[derive(Debug, Clone, Default)]
pub struct QtoknTp {
  pub segmn: String,
  pub instn: usize,
  pub qlkey: String,
  pub qlval: String
}

// query_content_inbatch.rs - Start batch process to perform queries into IDOC files
// stored in JSON format (2021-07-01 bar8tl)
pub fn query_content_inbatch(dbopt: &String, st: StageTp, idoct: &String) {}
/*  let f = File::open(&objtp).expect("Query JSON file not found.");
  let reqqy: RquryTp = from_reader(f).expect("JSON not well-formed");
  let flist: Vec<FilelistTp> = files_infolder(inpdr, inptp, objtp);
  for fle in &flist {
    let rtncd = query_content_onefile(inpdr, outdr, idt, objtp, refdr, fle, &reqqy);
    if wkflw == "yes" {
      next_stage(&rtncd, inpdr, outdr, pcddr, fle, ifilt);
    }
  }
} */

// query_content_onefile.rs - Perform query on individual IDOC files in JSON format
// (2021-07-01 bar8tl)
pub fn query_content_onefile(cnn: &Connection, st: &StageTp, fl: &FilelistTp) ->
  String { return OKAY.to_string(); }
/*
  let mut resqy: SquryTp = Default::default(return OKAY.to_string(););

//  let mut token: Vec<QtoknTp> = Default::default();
//  let field: String = Default::default();
  for fld in &reqqy.fields {
    let tokn: Vec<&str> = fld.split('\\').collect();
    if tokn.len() == 1 {
      resqy.fields.push(FieldTp{key: fld.to_string(), val: String::new()});
      continue;
    }

    if tokn.len() == 2 && tokn[0] == "CONTROL" {
      resqy.fields.push(FieldTp{key: fld, val: query_control(tokn[1]});
      continue;
    }
    for (i, t) in tokn.iter().enumerate() {
      if i < tokn.len()-1 {
        let c = split_querykey(t.to_string());
        println!("{:?}|", c);
        token.push(c);
      } else {
        field = tokn[tokn.len()-1].to_string();
        println!("{}", field);
        if token.len() == 1 {
          resqy.fields.push(FieldTp{key: fld, val: query_segment(token[0], field)});
          continue;
        }
      }
    }

  }
  let mut file = File::create(format!("{}_resp.json", fle.flnam)).expect("error");
  let fdata = serde_json::to_string_pretty(&resqy).unwrap();
  let bdata: &[u8] = fdata.as_bytes();
  file.write_all(&bdata).unwrap();
  return OKAY.to_string();
}

// split_querykey.rs -  Function to identify individual tokens in IDoc query key
// (2021-07-01 bar8tl)
pub fn split_querykey(key: String) -> QtoknTp {
  let mut q: QtoknTp = Default::default();
  let atokn: Vec<&str> = key.splitn(2, "[").collect();
  if atokn.len() == 2 {
    q.segmn = atokn[0].to_string();
    let btokn: Vec<&str> = atokn[1].splitn(2, "]").collect();
    if btokn.len() == 2 {
      q.instn = btokn[0].parse::<usize>().unwrap();
      let ctokn: Vec<&str> = btokn[1].splitn(2, ".").collect();
      if ctokn.len() == 2 {
        q.segmn = ctokn[0].to_string();
        let dtokn: Vec<&str> = ctokn[1].splitn(2, ":").collect();
        if dtokn.len() == 2 {
          q.qlkey = dtokn[0].to_string();
          q.qlval = dtokn[1].to_string();
        }
      }
    }
  } else {
    let btokn: Vec<&str> = key.splitn(2, ".").collect();
    if btokn.len() == 2 {
      q.segmn = btokn[0].to_string();
      let ctokn: Vec<&str> = btokn[1].splitn(2, ":").collect();
      if ctokn.len() == 2 {
        q.qlkey = ctokn[0].to_string();
        q.qlval = ctokn[1].to_string();
      }
    } else {
      q.segmn = key;
    }
  }
  return q;
}

func (d *Query_tp) MatchSegmL0(l0 int, sgkey lib.Qtokn_tp) (bool) {
  if d.Segm.Child[l0].Segmn == sgkey.Segmn {
    if sgkey.Instn != 0 && d.Segm.Child[l0].Instn == sgkey.Instn {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Qlval == sgkey.Qlval {
          return true
        }
      }
    } else {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Qlval == sgkey.Qlval {
          return true
        }
      } else {
        return true
      }
    }
  } else {
    return true
  }
  return false
}

func (d *Query_tp) MatchSegmL1(l0 int, sgkey lib.Qtokn_tp) (bool) {
  MatchSegmL0(0, sgkey)

  if d.Segm.Child[l0].Segmn == sgkey.Segmn {
    if sgkey.Instn != 0 && d.Segm.Child[l0].Child[l1].Instn == sgkey.Instn {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Child[l1].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Child[l1].Qlval == sgkey.Qlval {
          return true
        }
      }
    } else {
      if sgkey.Qlkey != "" && d.Segm.Child[l0].Child[l1].Qlkey == sgkey.Qlkey {
        if sgkey.Qlval != "" && d.Segm.Child[l0].Child[l1].Qlval == sgkey.Qlval {
          return true
        }
      } else {
        return true
      }
    }
  } else {
    return true
  }
  return false
}

// query_control.rs - Read specific field into Control Record (2021-07-01 bar8tl)
fn query_control(key: String) -> String {
  fc, _ := ioutil.ReadFile("control.json")
  json.Unmarshal(fc, &d.Cntrl)
  for c := range d.Cntrl.Field {
    if c.key == key {
      return c.val
    }
  }
  return (String::new());
}

func (d *Query_tp) QuerySegment(sgkey lib.Qtokn_tp, key string) (string) {
  fs, _ := ioutil.ReadFile("segment.json")
  json.Unmarshal(fs, &d.Segm)
  if d.MatchSegmL0(0, sgkey) {
    for _, f := range d.Segm.Child[0].Field {
      if f.Key == key {
        return f.Val
      }
    }
  }
  return ""
}
*/