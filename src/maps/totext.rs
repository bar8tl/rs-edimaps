// totxt.rs - Function modules being used to convert MS Excel mapping specification
// files to flat text mode format (2021-07-01 bar8tl)
use crate::config::MapsTp;
use crate::maps::proc_maps::{CrTp, IdxdatTp};
use std::fs::File;
use std::io::Write;

pub fn isrt_crhdr_text(cr: &CrTp, lstup: &String, s: &mut String, map: &MapsTp) {
  sprint (s, "BEGIN_MAPPING_SPECS");
  sprint (s, "  BEGIN_HEADER_RECORD");
  sprintf(s, "    TITLE               ", cr.hdr.mptit.as_str(), map);
  sprintf(s, "    LAST_UPDATE         ", lstup, map);
  sprintf(s, "    AUTHOR              ", cr.hdr.authr.as_str(), map);
  sprintf(s, "    VERSION             ", cr.hdr.bvers.as_str(), map);
  sprintf(s, "    CUSTOMER            ", cr.hdr.custm.as_str(), map);
  sprintf(s, "    TARGET_FORMAT       ", cr.hdr.tform.as_str(), map);
  sprintf(s, "    SOURCE_FORMAT       ", cr.hdr.sform.as_str(), map);
  sprint (s, "  END_HEADER_RECORD");
}

pub fn isrt_cregrp_text(cr: &CrTp, s: &mut String, map: &MapsTp) {
  if cr.ixgrp > 1 {
    sprint (s, "      END_FIELDS");
    sprint (s, "    END_SEGMENT");
    sprint (s, "  END_GROUP");
  }
  sprintf(s, "  BEGIN_GROUP           ", cr.ingrp.as_str(), map);
}

pub fn isrt_crgrps_text(cl: &[String; 7], cr: &CrTp, s: &mut String, map: &MapsTp) {
  sprint (s, "      END_FIELDS");
  sprint (s, "    END_SEGMENT");
  sprint (s, "  END_GROUP");
  sprintf(s, "  BEGIN_GROUP           ", cr.ingrp.as_str(), map);
  sprintf(s, "    TEXT                ", cl[3].as_str(), map);
  sprintf(s, "    LOOP_MAX            ", cl[4].as_str(), map);
  sprintf(s, "    STATUS              ", cl[5].as_str(), map);
  sprintf(s, "    DESCR               ", cl[0].as_str(), map);
  sprintf(s, "    CHANGE              ", cl[1].as_str(), map);
}

pub fn isrt_crsgms_text(cl: &[String; 7], cr: &CrTp, sgmtp: &String, s: &mut String,
  map: &MapsTp) {
  if cr.ixsgm > 1 {
    sprint (s, "      END_FIELDS");
    sprint (s, "    END_SEGMENT");
  }
  sprintf(s, "    BEGIN_SEGMENT       ", cr.insgm.as_str(), map);
  sprintf(s, "      NAME              ", sgmtp.as_str(), map);
  sprintf(s, "      LOOP_MAX          ", cl[4].as_str(), map);
  sprintf(s, "      STATUS            ", cl[5].as_str(), map);
  sprintf(s, "      DESCR             ", cl[0].as_str(), map);
  sprintf(s, "      CHANGE            ", cl[1].as_str(), map);
  sprint (s, "      BEGIN_FIELDS");
}

pub fn isrt_crflds_text(cl: &[String; 7], cr: &CrTp, s: &mut String, map: &MapsTp) {
  if cr.ixfld > 1 {
    sprint (s, "");
  }
  let wfld = if cl[2] == "" { "<empty>".to_string() } else { cl[2].clone() };
  sprintf(s, "        FIELD           ", wfld.as_str(), map);
  sprintf(s, "        SOURCE          ", cl[3].as_str(), map);
  sprintf(s, "        RULE_COND       ", cl[4].as_str(), map);
  sprintf(s, "        COMMENT         ", cl[5].as_str(), map);
  sprintf(s, "        SAMPLE          ", cl[6].as_str(), map);
  sprintf(s, "        TEXT            ", cl[0].as_str(), map);
  sprintf(s, "        CHANGE          ", cl[1].as_str(), map);
}

pub fn write_cr_text(bkpdr: &String, d: &IdxdatTp, s: &mut String) {
  sprint (s, "      END_FIELDS");
  sprint (s, "    END_SEGMENT");
  sprint (s, "  END_GROUP");
  sprint (s, "END_MAPPING_SPECS");
  let ofnam = d.fname.replace(".xlsx", ".txt");
  let mut file = File::create(format!("{}{}", bkpdr, ofnam)).expect("error");
  write!(file, "{}", s).unwrap();
}

pub fn sprint (line: &mut String, text: &str) {
  *line = format!("{}{}\n", line, text);
}

pub fn sprintf(line: &mut String, text: &str, value: &str, map: &MapsTp) {
  if value.len() == 0 {
    if map.nodat == "yes" {
      *line = format!("{}{}{}\n", line, text, map.ndchr);
    } else
    if map.omite == "no"  {
      *line = format!("{}{}{}\n", line, text, value);
    }
  } else {
    *line = format!("{}{}{}\n", line, text, value);
  }
}
