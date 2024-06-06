// tojson.rs - Function modules being used to convert MS Excel mapping specification
// files to json text mode format(2021-07-01 bar8tl)
use crate::maps::proc_maps::{CrTp, IdxdatTp};
use serde::Serialize;
use std::fs::File;
use std::io::Write;

// SpecsTp - Json file structure
#[derive(Debug, Clone, Default, Serialize)]
pub struct SpecsTp {
  pub header: HeaderTp,
  pub groups: Vec<GroupTp>
}

// HeaderTp - Header fields
#[derive(Debug, Clone, Default, Serialize)]
pub struct HeaderTp {
  pub title        : String,   // Title
  pub last_update  : String,   // Last Update
  pub author       : String,   // Author
  pub version      : String,   // Version / Change number
  pub customer     : String,   // Customer
  pub target_format: String,   // Target Format
  pub source_format: String    // Source Format
}

// GroupTp - Group fields
#[derive(Debug, Clone, Default, Serialize)]
pub struct GroupTp {
  #[serde(default)]
  pub group   : String,        // Group name
  #[serde(skip_serializing_if = "String::is_empty")]
  pub text    : String,        // Text
  #[serde(skip_serializing_if = "String::is_empty")]
  pub loop_max: String,        // Loop Max
  #[serde(skip_serializing_if = "String::is_empty")]
  pub status  : String,        // Status
  #[serde(skip_serializing_if = "String::is_empty")]
  pub descr   : String,        // Description
  #[serde(skip_serializing_if = "String::is_empty")]
  pub change  : String,        // Change
  pub segments: Vec<SegmentTp> // Child segments
}

// SegmentTp - Segment fields
#[derive(Debug, Clone, Default, Serialize)]
pub struct SegmentTp {
  pub segment : String,        // Segment id
  #[serde(skip_serializing_if = "String::is_empty")]
  pub name    : String,        // Segment name
  #[serde(skip_serializing_if = "String::is_empty")]
  pub loop_max: String,        // Loop max
  #[serde(skip_serializing_if = "String::is_empty")]
  pub status  : String,        // Status
  #[serde(skip_serializing_if = "String::is_empty")]
  pub descr   : String,        // Description
  #[serde(skip_serializing_if = "String::is_empty")]
  pub change  : String,        // Change
  pub fields  : Vec<FieldTp>   // Child fields
}

// FieldTp - Field fields
#[derive(Debug, Clone, Default, Serialize)]
pub struct FieldTp {
  pub field  : String,         // Field name (Target)
  #[serde(skip_serializing_if = "String::is_empty")]
  pub source : String,         // Source
  #[serde(skip_serializing_if = "String::is_empty")]
  pub r_cond : String,         // Rule or Condition
  #[serde(skip_serializing_if = "String::is_empty")]
  pub comment: String,         // Comment (field description)
  #[serde(skip_serializing_if = "String::is_empty")]
  pub sample : String,         // Sample
  #[serde(skip_serializing_if = "String::is_empty")]
  pub text   : String,         // Text
  #[serde(skip_serializing_if = "String::is_empty")]
  pub change : String          // Change
}

pub fn init_cr_json(sp: &mut SpecsTp) {
  *sp = SpecsTp { ..Default::default() };
}

pub fn isrt_crhdr_json(cr: &CrTp, lstup: &String, sp: &mut SpecsTp) {
  sp.header = HeaderTp {
    title        : cr.hdr.mptit.clone(),
    last_update  : lstup.to_string(),
    author       : cr.hdr.authr.clone(),
    version      : cr.hdr.bvers.clone(),
    customer     : cr.hdr.custm.clone(),
    target_format: cr.hdr.tform.clone(),
    source_format: cr.hdr.sform.clone()
  };
}

pub fn isrt_cregrp_json(cr: &CrTp, sp: &mut SpecsTp) {
  sp.groups.push( GroupTp {
    group   : cr.ingrp.clone(),
    text    : String::new(),
    loop_max: String::new(),
    status  : String::new(),
    descr   : String::new(),
    change  : String::new(),
    segments: Vec::new()
  } );
}

pub fn isrt_crgrps_json(cl: &[String; 7], cr: &CrTp, sp: &mut SpecsTp) {
  sp.groups.push( GroupTp {
    group   : cr.ingrp.clone(),
    text    : cl[3].clone(),
    loop_max: cl[4].clone(),
    status  : cl[5].clone(),
    descr   : cl[0].clone(),
    change  : cl[1].clone(),
    segments: Vec::new()
  } );
}

pub fn isrt_crsgms_json(cl: &[String; 7], cr: &CrTp, sgmtp: &String,
  sp: &mut SpecsTp) {
  sp.groups[cr.ixgrp as usize-1].segments.push( SegmentTp {
    segment : cr.insgm.clone(),
    name    : sgmtp.to_string(),
    loop_max: cl[4].clone(),
    status  : cl[5].clone(),
    descr   : cl[0].clone(),
    change  : cl[1].clone(),
    fields  : Vec::new()
  } );
}

pub fn isrt_crflds_json(cl: &[String; 7], cr: &CrTp, sp: &mut SpecsTp) {
  //println!("|{}|{}|", cr.ixgrp, cr.ixsgm);
  let wfld = if cl[2] == "" { "<empty>".to_string() } else { cl[2].clone() };
  sp.groups[cr.ixgrp as usize-1].segments[cr.ixsgm as usize-1].fields.push(FieldTp {
    field  : wfld,
    source : cl[3].clone(),
    r_cond : cl[4].clone(),
    comment: cl[5].clone(),
    sample : cl[6].clone(),
    text   : cl[0].clone(),
    change : cl[1].clone()
  } );
}

pub fn write_cr_json(bkpdr: &String, d: &IdxdatTp, sp: &SpecsTp) {
  let ofnam = d.fname.replace(".xlsx", ".json");
  let mut file = File::create(format!("{}{}", bkpdr, ofnam)).expect("error");
  let fdata = serde_json::to_string_pretty(sp).unwrap();
  let bdata: &[u8] = fdata.as_bytes();
  file.write_all(&bdata).unwrap();
}
