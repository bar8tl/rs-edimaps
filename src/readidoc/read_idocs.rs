// read_idocs.rs - Main logic to process SAP IDoc content processing/workflow.
// Starting by getting workflow settings (2021-07-01 bar8tl)
use crate::assets::{IdoctpTp, StepTp};
use crate::config::WkflowTp;
use crate::readidoc::tofixdsz::{flat_content_onefile, flat_content_inbatch};
use crate::readidoc::tojson::{json_content_onefile, json_content_inbatch};
use crate::readidoc::runquery::{query_content_onefile, query_content_inbatch};
use rblib::files_infolder::FilelistTp;
use rblib::pass_filter::pass_filter;
use rblib::move_file_wf::move_file_wf;
use rblib::rename_file_wf::rename_file_wf;
use rusqlite::Connection;

pub const OKAY: &str = "00";
pub const INP : &str = "inp";
pub const OUT : &str = "out";

#[derive(Debug, Clone, Default)]
pub struct StageTp {
  pub wfhdr: WkflowTp,
  pub wfstp: StepTp
}

pub fn read_idocs(dbopt: &String, step: &str, wkflow: &WkflowTp, file: &String,
  single: bool) {
  let mut st: StageTp = Default::default();
  st.wfhdr.cntrl = wkflow.cntrl.clone();
  st.wfhdr.clien = wkflow.clien.clone();
  st.wfhdr.rcvpf = wkflow.rcvpf.clone();
  let cnn = Connection::open(dbopt).expect("DB Error");
  cnn.query_row("SELECT * FROM wkflow WHERE step=?1;", [step,], |row| { Ok({
    st.wfstp.step  = row.get(0).unwrap();
    st.wfstp.inpdr = row.get(1).unwrap();
    st.wfstp.inptp = row.get(2).unwrap();
    st.wfstp.outdr = row.get(3).unwrap();
    st.wfstp.outtp = row.get(4).unwrap();
    st.wfstp.refdr = row.get(5).unwrap();
    st.wfstp.reftp = row.get(6).unwrap();
    st.wfstp.wkflw = row.get(7).unwrap();
    st.wfstp.pcddr = row.get(8).unwrap();
    st.wfstp.ifilt = row.get(9).unwrap();
  }) }).expect("Error: Step type not found in repository");
  if single {
    let atokn: Vec<&str> = file.rsplitn(2, ".").collect();
    let fl = FilelistTp {
      flpth: format!("{}{}", st.wfstp.inpdr, file),
      fldir: st.wfstp.inpdr.clone(),
      flide: file.clone(),
      flnam: atokn[1].to_string(),
      flext: atokn[0].to_string()
    };
    if step == "fixed" {
      flat_content_onefile (&cnn, &st, &fl);
    } else if step == "json" {
      json_content_onefile (&cnn, &st, &fl);
    } else if step == "query" {
      query_content_onefile(&cnn, &st, &fl);
    }
  } else {
    if step == "fixed" {
      flat_content_inbatch (dbopt, st, file);
    } else if step == "json" {
      json_content_inbatch (dbopt, st, file);
    } else if step == "query" {
      query_content_inbatch(dbopt, st, file);
    }
  }
}

// get_idoctp.rs - Upload internal table of IDOC types (2021-07-01 bar8tl)
pub fn get_idoctp(cnn: &Connection, wkflow: &WkflowTp, flide: &String) -> IdoctpTp {
  let mut it: IdoctpTp = Default::default();
  let atokn: Vec<&str> = flide.splitn(2, "_").collect();
  if atokn.len() == 2 {
    let short = atokn[0].to_string();
    cnn.query_row("SELECT * FROM idoctp WHERE short=?1;", [short,], |row| { Ok({
      it.itype = row.get(0).unwrap();
      it.itype = it.itype.to_uppercase();
      it.idefn = it.itype.to_uppercase().replace("/", "_-");
      it.short = row.get(1).unwrap();
      it.cntrl = wkflow.cntrl.clone();
      it.clien = wkflow.clien.clone();
      it.rcvpf = row.get(2).unwrap();
      if it.rcvpf.len() == 0 {
        it.rcvpf = wkflow.rcvpf.clone();
      }
    }) }).expect("Error: Idoctp not found in repository");
  }
  return it;
}

// next_stage.rs - Conclude workflow steps process (2021-07-01 bar8tl)
pub fn next_stage(rtncd: &String, st: &StageTp, fl: &FilelistTp) {
  if st.wfstp.ifilt.len() == 0 || (st.wfstp.ifilt.len() > 0 &&
    pass_filter(&st.wfstp.ifilt, &fl.flnam)) {
    if rtncd == OKAY {
      move_file_wf(INP, st.wfstp.inpdr.as_str(), st.wfstp.pcddr.as_str(), &fl.flnam,
        &fl.flext);
      rename_file_wf(OUT, st.wfstp.outdr.as_str(), &fl.flnam, &fl.flext);
    }
  }
}
