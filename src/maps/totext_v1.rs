// totxt.rs - Function modules being used to convert MS Excel mapping specification
// files to flat text mode format (2021-07-01 bar8tl)
use crate::maps::proc_maps::CrTp;
use std::fs::File;
use std::io::Write;

pub fn fprint (w: &mut File, text: &str) {
  writeln!(w, "{}", text).unwrap();
}

pub fn fprintf(w: &mut File, text: &str, value: &str) {
  if value.len() > 0 {
    writeln!(w, "{}{}", text, value).unwrap();
  }
}

pub fn isrt_crhdr_text(cr: &CrTp, lstup: &String, w: &mut File) {
  fprint (w, "BEGIN_MAPPING_SPECS");
  fprint (w, "  BEGIN_HEADER_RECORD");
  fprintf(w, "    TITLE               ", cr.hdr.mptit.as_str());
  fprintf(w, "    LAST_UPDATE         ", lstup);
  fprintf(w, "    AUTHOR              ", cr.hdr.authr.as_str());
  fprintf(w, "    VERSION             ", cr.hdr.bvers.as_str());
  fprintf(w, "    CUSTOMER            ", cr.hdr.custm.as_str());
  fprintf(w, "    TARGET_FORMAT       ", cr.hdr.tform.as_str());
  fprintf(w, "    SOURCE_FORMAT       ", cr.hdr.sform.as_str());
  fprint (w, "  END_HEADER_RECORD");
}

pub fn isrt_cregrp_text(cr: &CrTp, w: &mut File) {
  if cr.ixgrp > 1 {
    fprint (w, "      END_FIELDS");
    fprint (w, "    END_SEGMENT");
    fprint (w, "  END_GROUP");
  }
  fprintf(w, "  BEGIN_GROUP           ", cr.ingrp.as_str());
}

pub fn isrt_crgrps_text(cl: &[String; 7], cr: &CrTp, w: &mut File) {
  fprint (w, "      END_FIELDS");
  fprint (w, "    END_SEGMENT");
  fprint (w, "  END_GROUP");
  fprintf(w, "  BEGIN_GROUP           ", cr.ingrp.as_str());
  fprintf(w, "    TEXT                ", cl[3].as_str());
  fprintf(w, "    LOOP_MAX            ", cl[4].as_str());
  fprintf(w, "    STATUS              ", cl[5].as_str());
  fprintf(w, "    DESCR               ", cl[0].as_str());
  fprintf(w, "    CHANGE              ", cl[1].as_str());
}

pub fn isrt_crsgms_text(cl: &[String; 7], cr: &CrTp, sgmtp: &String, w: &mut File) {
  if cr.ixsgm > 1 {
    fprint (w, "      END_FIELDS");
    fprint (w, "    END_SEGMENT");
  }
  fprintf(w, "    BEGIN_SEGMENT       ", cr.insgm.as_str());
  fprintf(w, "      NAME              ", sgmtp.as_str());
  fprintf(w, "      LOOP_MAX          ", cl[4].as_str());
  fprintf(w, "      STATUS            ", cl[5].as_str());
  fprintf(w, "      DESCR             ", cl[0].as_str());
  fprintf(w, "      CHANGE            ", cl[1].as_str());
  fprint (w, "      BEGIN_FIELDS");
}

pub fn isrt_crflds_text(cl: &[String; 7], cr: &CrTp, w: &mut File) {
  if cr.ixfld > 1 {
    fprint (w, "");
  }
  let wfld = if cl[2] == "" { "<empty>".to_string() } else { cl[2].clone() };
  fprintf(w, "        FIELD           ", wfld.as_str());
  fprintf(w, "        SOURCE          ", cl[3].as_str());
  fprintf(w, "        RULE_COND       ", cl[4].as_str());
  fprintf(w, "        COMMENT         ", cl[5].as_str());
  fprintf(w, "        SAMPLE          ", cl[6].as_str());
  fprintf(w, "        TEXT            ", cl[0].as_str());
  fprintf(w, "        CHANGE          ", cl[1].as_str());
}

pub fn write_cr_text(w: &mut File) {
  fprint (w, "      END_FIELDS");
  fprint (w, "    END_SEGMENT");
  fprint (w, "  END_GROUP");
  fprint (w, "END_MAPPING_SPECS");
}
